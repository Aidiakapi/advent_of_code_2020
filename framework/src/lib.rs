#![feature(
    auto_traits,
    const_fn_fn_ptr_basics,
    const_fn_transmute,
    const_fn,
    const_panic,
    const_ptr_offset,
    const_raw_ptr_deref,
    negative_impls,
    never_type
)]

pub use paste::paste;

pub mod bootstrap;
pub mod error;
pub mod iter;
pub mod num;
pub mod parser;
pub mod test;
pub mod traits;

use arrayvec::ArrayVec;
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};
use traits::ResultWhereValueIsErrorExt;

pub fn run(days: &[(&'static str, &'static dyn traits::Day)]) -> Result<(), error::Error> {
    use colored::Colorize;
    println!(
        "{} {} {} {}",
        "Advent".bright_red().bold(),
        "of".bright_white(),
        "Code".bright_green().bold(),
        "2020".bright_blue()
    );

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
        let mut input = get_day_input(throttle, session_key, day_nr)?;

        if input.chars().last() == Some('\n') {
            input.pop();
        }

        let start_time = std::time::Instant::now();
        let results = day.evaluate(input);
        let duration = std::time::Instant::now() - start_time;

        const VALUE_ALIGNMENT: usize = 16;

        let results: ArrayVec<[_; 2]> = results
            .into_iter()
            .map(|(pt_name, result)| (pt_name, result.map_err(|err| err.to_string())))
            .collect();
        let use_expanded_format = results
            .iter()
            .any(|(_, result)| result.unwrap_either().contains('\n'));

        let duration_nanos = duration.as_nanos();
        let duration_ms = duration_nanos / 1_000_000;
        let duration_decimals = duration_nanos / 1_000 % 1_000 / 10;

        print!(
            "{} ({} ms)",
            format!("day{:0>2}", day_nr).bright_blue(),
            format!("{: >3}.{:0>2}", duration_ms, duration_decimals).bright_white(),
        );
        if use_expanded_format {
            println!();
        } else {
            print!(" |");
        }

        for (pt_name, result) in results {
            let pt_ident = pt_name.bright_green();
            let value = match result {
                Ok(value) => value.bright_white().bold(),
                Err(err) => err.bright_red().bold().underline(),
            };
            if use_expanded_format {
                println!("{}\n{}", pt_ident, value);
            } else {
                print!(" {} {:>width$} |", pt_ident, value, width = VALUE_ALIGNMENT);
            }
        }
        if !use_expanded_format {
            println!();
        }
    }

    Ok(())
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

fn is_valid_input(input: &str) -> bool {
    input
        .chars()
        .all(|c| c.is_ascii() && !c.is_ascii_control() || c == '\n')
}

fn get_day_input(
    throttle: &mut RequestThrottle,
    session_key: &mut SessionKey,
    day_nr: u32,
) -> Result<String, error::Error> {
    let file_path = get_day_input_path(day_nr);
    if let Some(contents) = fs::read(&file_path)
        .ok()
        .and_then(|input| String::from_utf8(input).ok())
        .filter(|input| is_valid_input(&input))
    {
        return Ok(contents);
    }

    let now = Instant::now();
    if let Some(previous_request_time) = throttle.0 {
        let delta = now - previous_request_time;
        if delta < DELAY_BETWEEN_REQUESTS {
            thread::sleep(DELAY_BETWEEN_REQUESTS - delta);
        }
    }

    let session_key = get_session_key(session_key)?;
    throttle.0 = Some(now);

    let resp = ureq::get(&format!(
        "https://adventofcode.com/2020/day/{}/input",
        day_nr
    ))
    .set("cookie", &format!("session={}", session_key))
    .timeout(Duration::from_secs(5))
    .call();

    if let Some(_err) = resp.synthetic_error() {
        return Err(error::Error::NetworkError);
    }

    let mut contents = Vec::new();
    resp.into_reader().read_to_end(&mut contents)?;
    let contents = String::from_utf8(contents)
        .ok()
        .filter(|input| is_valid_input(&input))
        .ok_or(error::Error::InvalidInput("invalid characters in input"))?;

    let mut dir_path = file_path.clone();
    dir_path.pop();
    let _ = fs::create_dir_all(dir_path);
    let _ = fs::write(file_path, &contents);

    Ok(contents)
}
