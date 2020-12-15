use crate::prelude::*;

day!(10, parse => pt1, pt2);

pub fn pt1(input: &[u32]) -> u32 {
    let input = add_endpoints_and_sort(input);
    let (delta1, delta3) =
        input
            .array_windows()
            .fold((0, 0), |acc: (u32, u32), &[a, b]: &[u32; 2]| match b - a {
                1 => (acc.0 + 1, acc.1),
                3 => (acc.0, acc.1 + 1),
                _ => acc,
            });
    delta1 * delta3
}

pub fn pt2(input: &[u32]) -> u64 {
    let input = add_endpoints_and_sort(input);
    let mut cache = vec![None; input.len()];
    fn calculate_possible_path_count(cache: &mut [Option<u64>], slice: &[u32]) -> u64 {
        if slice.len() <= 1 {
            return 1;
        }
        if let Some(value) = cache[slice.len() - 1] {
            return value;
        }
        let from = slice[0];
        let mut possible_paths = 0u64;
        for i in 1..slice.len() {
            let delta = slice[i] - from;
            if delta > 3 {
                break;
            }
            possible_paths += calculate_possible_path_count(cache, &slice[i..]);
        }
        cache[slice.len() - 1] = Some(possible_paths);
        possible_paths
    }

    calculate_possible_path_count(&mut cache, &input)
}

fn add_endpoints_and_sort(input: &[u32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(input.len() + 2);
    result.push(0);
    result.extend_from_slice(input);
    result.sort_unstable();
    result.push(result[result.len() - 1] + 3);
    result
}

pub fn parse(input: &str) -> Result<Vec<u32>> {
    use framework::parser::*;
    separated_list1(char('\n'), take_u32)(input).into_result()
}

#[cfg(test)]
const EXAMPLE: &str = "\
16
10
15
5
1
11
7
19
6
12
4";
#[cfg(test)]
const LARGER_EXAMPLE: &str = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

standard_tests!(
    parse []
    pt1 [
        EXAMPLE => { 7 * 5 }
        LARGER_EXAMPLE => { 22 * 10 }
    ]
    pt2 [
        EXAMPLE => 8
        LARGER_EXAMPLE => 19208
    ]
);
