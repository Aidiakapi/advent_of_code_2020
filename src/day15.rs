use crate::prelude::*;

day!(15, parse => pt1, pt2);

fn pts<const GOAL_TURN: u32>(input: &[u32]) -> u32 {
    let mut last_seen_at = HashMap::new();

    for (i, &number) in input.iter().enumerate().take(input.len() - 1) {
        last_seen_at.insert(number, (i + 1) as u32);
    }

    let mut previous_number = *input.last().unwrap();
    let mut previous_turn = input.len() as u32;

    loop {
        // Consider the last number, and the time we've seen it before then
        let current_turn_number = last_seen_at
            .get(&previous_number)
            .map(|&v| previous_turn - v)
            .unwrap_or(0);
        last_seen_at.insert(previous_number, previous_turn);
        previous_turn += 1;
        if previous_turn == GOAL_TURN {
            return current_turn_number;
        }
        previous_number = current_turn_number;
    }
}

pub fn pt1(input: &[u32]) -> u32 {
    pts::<2020>(input)
}

pub fn pt2(input: &[u32]) -> u32 {
    pts::<30000000>(input)
}

pub fn parse(input: &str) -> Result<Vec<u32>> {
    use framework::parser::*;
    separated_list1(char(','), take_u32)(input).into_result()
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
