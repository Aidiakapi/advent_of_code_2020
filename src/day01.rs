use crate::prelude::*;
day!(1, parse => pt1, pt2);

/// Finds two numbers that sum to a target value, and returns the product of
/// those two numbers. The input list must be sorted.
fn find_product_of_numbers_that_sum_to_target(target: u32, sorted_list: &[u32]) -> Option<u32> {
    if sorted_list.is_empty() {
        return None;
    }

    // The lower bound cannot be below the (target - highest value), because
    // then the sum would always be lower than the target...
    let lower_bound = sorted_list
        .binary_search(&(target - sorted_list[sorted_list.len() - 1].min(target)))
        .unwrap_either();
    // ...and the upper bound similarly cannot be above (target - lowest value),
    // because then the sum would always exceed the target number.
    let upper_bound = sorted_list
        .binary_search(&(target - sorted_list[lower_bound].min(target)))
        .unwrap_either()
        .min(sorted_list.len() - 1);

    let slice = &sorted_list[lower_bound..=upper_bound];
    let cutoff_point = (target + 1) / 2;
    for (i, a) in slice.iter().cloned().enumerate() {
        // Once a has become greater than half of the total, no sum of it and
        // a larger number can be equal to or lower than the target.
        if a >= cutoff_point {
            break;
        }
        let b = target - a;
        if let Ok(_) = slice[i + 1..].binary_search(&b) {
            return Some(a * b);
        }
    }
    None
}

pub fn pt1(input: &Vec<u32>) -> Result<u32> {
    find_product_of_numbers_that_sum_to_target(2020, input)
        .ok_or_else(|| Error::InvalidInput("No solution".to_owned()))
}

pub fn pt2(input: &Vec<u32>) -> Result<u32> {
    for (i, a) in input.iter().cloned().enumerate() {
        if a > 2020 {
            break;
        }
        if let Some(product) = find_product_of_numbers_that_sum_to_target(2020 - a, &input[i + 1..])
        {
            return Ok(product * a);
        }
    }
    Err(Error::InvalidInput("No solution".to_owned()))
}

pub fn parse(input: &[u8]) -> Result<Vec<u32>> {
    use framework::parser::*;
    separated_list(newline, take_u32)(input)
        .into_result()
        .map(|mut input| {
            input.sort();
            input
        })
}

standard_tests!(
    parse [
        b"1721\n979\n366\n299\n675\n1456" => vec![299, 366, 675, 979, 1456, 1721]
    ]
    pt1 [
        b"1721\n979\n366\n299\n675\n1456" => 514579
    ]
    pt2 [
        b"1721\n979\n366\n299\n675\n1456" => 241861950
    ]
);