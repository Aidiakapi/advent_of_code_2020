use crate::prelude::*;
use std::num::NonZeroUsize;

day!(15, parse => pt1, pt2);

fn pts<const GOAL_TURN: usize>(input: &[usize]) -> usize {
    let mut last_seen_at = vec![None; GOAL_TURN as usize];

    for (i, &number) in input.iter().enumerate().take(input.len() - 1) {
        last_seen_at[number] = NonZeroUsize::new(i + 1);
    }

    let mut previous_number = *input.last().unwrap();
    let mut previous_turn = input.len();

    loop {
        // Consider the last number, and the time we've seen it before then
        let current_turn_number = last_seen_at[previous_number as usize]
            .map(|v| previous_turn - v.get())
            .unwrap_or(0);
        last_seen_at[previous_number] = NonZeroUsize::new(previous_turn);
        previous_turn += 1;
        if previous_turn == GOAL_TURN {
            return current_turn_number;
        }
        previous_number = current_turn_number;
    }
}

pub fn pt1(input: &[usize]) -> usize {
    pts::<2020>(input)
}

pub fn pt2(input: &[usize]) -> usize {
    pts::<30000000>(input)
}

pub fn parse(input: &str) -> Result<Vec<usize>> {
    use framework::parser::*;
    separated_list1(char(','), take_usize)(input).into_result()
}

standard_tests!(
    parse []
    pt1 [
        "0,3,6" => 436
        "1,3,2" => 1
        "2,1,3" => 10
        "1,2,3" => 27
        "2,3,1" => 78
        "3,2,1" => 438
        "3,1,2" => 1836
    ]
    pt2 []
);
