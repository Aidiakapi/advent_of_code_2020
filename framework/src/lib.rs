#![feature(
    const_fn_fn_ptr_basics,
    const_fn_transmute,
    const_fn,
    const_panic,
    const_ptr_offset,
    const_raw_ptr_deref,
    negative_impls,
    never_type,
    optin_builtin_traits
)]

pub use paste::paste;

pub mod ascii;
pub mod bootstrap;
pub mod error;
pub mod iter;
pub mod num;
pub mod parser;
pub mod test;
pub mod traits;
pub mod vec2;

use crate::ascii::*;

use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

pub fn run(days: &[(&'static str, &'static dyn traits::Day)]) {
    let session_key = &mut SessionKey::default();
    let throttle = &mut RequestThrottle::default();

    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    let args = args.iter().filter_map(|x| x.to_str()).collect::<Vec<_>>();
    let is_module_included = |name| args.len() == 0 || args.contains(name);

    for (module_name, day) in days {
        if !is_module_included(module_name) {
            continue;
        }

        let day_nr = day.nr();
        let mut input = match get_day_input(throttle, session_key, day_nr) {
            Some(x) => x,
            None => {
                eprintln!("couldn't get input for day {:0>2}", day_nr);
                continue;
            }
        };

        if input.last() == Some(achar::LineFeed) {
            input.pop();
        }

        let results = day.evaluate(input);
        for result in results {
            match result.1 {
                Ok(value) => {
                    println!("day{:0>2}::{}\n{}", day_nr, result.0, value);
                }
                Err(err) => {
                    eprintln!("day{:0>2}::{}\n{}", day_nr, result.0, err);
                }
            }
        }
    }
}

fn get_session_key(session_key: &mut SessionKey) -> io::Result<&str> {
    if session_key.0.is_none() {
        let key = fs::read_to_string("token.txt")?.trim().to_string();
        session_key.0 = Some(key);
    }
    Ok(session_key.0.as_deref().unwrap())
}

fn get_day_input_path(day_nr: u32) -> PathBuf {
    let mut path = PathBuf::new();
    path.push("inputs");
    path.push(format!("day{:0>2}.txt", day_nr));
    path
}

const DELAY_BETWEEN_REQUESTS: Duration = Duration::from_secs(3);

#[derive(Default)]
struct SessionKey(Option<String>);

#[derive(Default)]
struct RequestThrottle(Option<Instant>);

fn get_day_input(
    throttle: &mut RequestThrottle,
    session_key: &mut SessionKey,
    day_nr: u32,
) -> Option<AString> {
    let file_path = get_day_input_path(day_nr);
    if let Some(contents) = fs::read(&file_path)
        .ok()
        .and_then(|x| AString::from_ascii(x).ok())
    {
        return Some(contents);
    }

    let now = Instant::now();
    if let Some(previous_request_time) = throttle.0 {
        let delta = now - previous_request_time;
        if delta < DELAY_BETWEEN_REQUESTS {
            thread::sleep(DELAY_BETWEEN_REQUESTS - delta);
        }
    }

    let session_key = match get_session_key(session_key) {
        Ok(x) => x,
        Err(err) => {
            eprintln!(
                "couldn't read session key from file \"token.txt\":\n{}",
                err
            );
            return None;
        }
    };
    throttle.0 = Some(now);

    let resp = ureq::get(&format!(
        "https://adventofcode.com/2020/day/{}/input",
        day_nr
    ))
    .set("cookie", &format!("session={}", session_key))
    .timeout(Duration::from_secs(5))
    .call();

    if let Some(err) = resp.synthetic_error() {
        eprintln!("error in network request: {}", err);
        return None;
    }

    let mut contents = Vec::new();
    resp.into_reader().read_to_end(&mut contents).ok()?;
    let contents = match AString::from_ascii(contents) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("response contains invalid ASCII character(s)");
            return None;
        }
    };

    let mut dir_path = file_path.clone();
    dir_path.pop();
    let _ = fs::create_dir_all(dir_path);
    let _ = fs::write(file_path, &contents);

    Some(contents)
}
