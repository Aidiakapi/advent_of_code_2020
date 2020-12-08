use std::collections::{HashMap, HashSet};

use crate::prelude::*;
day!(7, parse => pt1, pt2);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Constraint<'s> {
    target: BagType<'s>,
    requires: Vec<(u32, BagType<'s>)>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BagType<'s> {
    tone: &'s str,
    color: &'s str,
}

pub fn pt1(constraints: &[Constraint]) -> usize {
    let mut potential_containers = HashSet::<BagType>::new();
    potential_containers.insert(BagType {
        tone: "shiny",
        color: "gold",
    });
    let mut additions = HashSet::<BagType>::new();
    loop {
        additions.extend(
            constraints
                .iter()
                .filter(|constraint| {
                    constraint
                        .requires
                        .iter()
                        .any(|(_, bag_type)| potential_containers.contains(bag_type))
                })
                .map(|constraint| constraint.target),
        );
        let old_len = potential_containers.len();
        potential_containers.extend(additions.iter());
        if old_len == potential_containers.len() {
            break;
        }
    }
    // Don't include the shiny gold bag we started with
    potential_containers.len() - 1
}

pub fn pt2(constraints: &[Constraint]) -> u64 {
    let constraints = constraints
        .iter()
        .cloned()
        .map(|constraint| (constraint.target, constraint.requires))
        .collect::<HashMap<_, _>>();

    let mut cache = HashMap::with_capacity(constraints.len());
    fn count_required_bags<'s, 'c>(
        constraints: &'s HashMap<BagType<'s>, Vec<(u32, BagType)>>,
        cache: &'c mut HashMap<BagType<'s>, u64>,
        bag_type: &'s BagType,
    ) -> u64
    where
        's: 'c,
    {
        if let Some(&cached_value) = cache.get(bag_type) {
            return cached_value;
        }
        let result = constraints[bag_type]
            .iter()
            .map(|(required_count, required_type)| {
                *required_count as u64
                    * (1 + count_required_bags(constraints, cache, required_type))
            })
            .sum();
        cache.insert(*bag_type, result);
        result
    }

    count_required_bags(
        &constraints,
        &mut cache,
        &BagType {
            tone: "shiny",
            color: "gold",
        },
    )
}

pub fn parse(input: &str) -> Result<Vec<Constraint>> {
    use framework::parser::*;
    fn bag_type(input: &str) -> IResult<BagType> {
        map(
            pair(
                terminated(alpha1, char(' ')),
                terminated(alpha1, pair(tag(" bag"), opt(char('s')))),
            ),
            |(tone, color)| BagType { tone, color },
        )(input)
    }
    let requirement = pair(terminated(take_u32, char(' ')), bag_type);
    let requirements = alt((
        separated_list1(tag(", "), requirement),
        map(tag("no other bags"), |_| Vec::new()),
    ));
    let constraint = map(
        pair(
            terminated(bag_type, tag(" contain ")),
            terminated(requirements, char('.')),
        ),
        |(target, requires)| Constraint { target, requires },
    );
    separated_list1(char('\n'), constraint)(input).into_result()
}

#[cfg(test)]
const COMMON_EXAMPLE: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

standard_tests!(
    parse []
    pt1 [ COMMON_EXAMPLE => 4 ]
    pt2 [ COMMON_EXAMPLE => 32
        "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags." => 126 ]
);
