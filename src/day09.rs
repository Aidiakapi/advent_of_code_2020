use crate::prelude::*;
use std::cmp::Ordering;

day!(9, parse => pt1, pt2);

fn find_first_invalid_number<const PREAMBLE_PLUS_ONE: usize>(input: &[u64]) -> Option<u64> {
    input
        .array_windows()
        .filter_map(|window: &[u64; PREAMBLE_PLUS_ONE]| {
            let target = window[PREAMBLE_PLUS_ONE - 1];
            if window[0..PREAMBLE_PLUS_ONE - 2]
                .iter()
                .enumerate()
                .any(|(i, &x)| {
                    window[i + 1..PREAMBLE_PLUS_ONE - 1]
                        .iter()
                        .any(|&y| x + y == target)
                })
            {
                None
            } else {
                Some(target)
            }
        })
        .next()
}

fn find_encryption_weakness(input: &[u64], invalid_number: u64) -> Option<u64> {
    let mut lower = 0;
    let mut upper = 2;
    let mut current_sum = input[0] + input[1];
    loop {
        match current_sum.cmp(&invalid_number) {
            Ordering::Equal => {
                let (&min, &max) = input[lower..upper].iter().minmax().into_option().unwrap();
                return Some(min + max);
            }
            Ordering::Greater if upper - lower > 2 => {
                current_sum -= input[lower];
                lower += 1;
            }
            _ => {
                if upper >= input.len() {
                    return None;
                }
                current_sum += input[upper];
                upper += 1;
            }
        }
    }
}

pub fn pt1(input: &[u64]) -> Result<u64> {
    find_first_invalid_number::<26>(input).ok_or(Error::NoSolution)
}

pub fn pt2(input: &[u64]) -> Result<u64> {
    find_encryption_weakness(input, pt1(input)?).ok_or(Error::NoSolution)
}

pub fn parse(input: &str) -> Result<Vec<u64>> {
    use framework::parser::*;
    separated_list1(char('\n'), take_u64)(input).into_result()
}

#[cfg(test)]
fn pt1_test(input: &[u64]) -> Result<u64> {
    find_first_invalid_number::<6>(input).ok_or(Error::NoSolution)
}

#[cfg(test)]
fn pt2_test(input: &[u64]) -> Result<u64> {
    find_encryption_weakness(input, pt1_test(input)?).ok_or(Error::NoSolution)
}

#[cfg(test)]
const EXAMPLE: &str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

standard_tests!(
    parse []
    pt1_test [ EXAMPLE => 127 ]
    pt2_test [ EXAMPLE => 62 ]
);
