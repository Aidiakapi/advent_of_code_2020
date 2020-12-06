use std::collections::{HashSet, HashMap};

use crate::prelude::*;
day!(6, parse => pt1, pt2);

pub fn pt1(groups: &[Vec<&astr>]) -> usize {
    let mut set = HashSet::new();
    groups.iter()
        .map(|group| {
            set.clear();
            set.extend(group.iter().flat_map(|person| person.chars()));
            set.len()
        })
        .sum()
}

pub fn pt2(groups: &[Vec<&astr>]) -> usize {
    let mut map = HashMap::new();
    groups.iter()
        .map(|group| {
            map.clear();
            for question in group.iter().flat_map(|person| person.chars()) {
                *map.entry(question).or_insert(0usize) += 1;
            }
            map.values().count_if(|&value| value == group.len())
        })
        .sum()
}

pub fn parse(input: &astr) -> Vec<Vec<&astr>> {
    use framework::parser::*;
    input
        .split_str(astr!(b"\n\n"))
        .map(|group| group.split(achar::LineFeed).collect())
        .collect()
}

#[cfg(test)]
const EXAMPLE: &[u8] = b"abc

a
b
c

ab
ac

a
a
a
a

b";

standard_tests!(
    parse []
    pt1 [ EXAMPLE => 11 ]
    pt2 [ EXAMPLE => 6 ]
);
