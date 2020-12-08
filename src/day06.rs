use std::collections::{HashMap, HashSet};

use crate::prelude::*;
day!(6, parse => pt1, pt2);

pub fn pt1(groups: &[Vec<&str>]) -> usize {
    let mut set = HashSet::new();
    groups
        .iter()
        .map(|group| {
            set.clear();
            set.extend(group.iter().flat_map(|person| person.chars()));
            set.len()
        })
        .sum()
}

pub fn pt2(groups: &[Vec<&str>]) -> usize {
    let mut map = HashMap::new();
    groups
        .iter()
        .map(|group| {
            map.clear();
            for question in group.iter().flat_map(|person| person.chars()) {
                *map.entry(question).or_insert(0usize) += 1;
            }
            map.values().count_if(|&value| value == group.len())
        })
        .sum()
}

pub fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .split("\n\n")
        .map(|group| group.split('\n').collect())
        .collect()
}

#[cfg(test)]
const EXAMPLE: &str = "abc

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
