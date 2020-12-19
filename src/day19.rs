use crate::prelude::*;
use arrayvec::ArrayVec;
use std::collections::hash_map::Entry;

day!(19, parse => pt1, pt2);

type Rules = HashMap<u32, Rule>;
type RulesRef = ArrayVec<[u32; 3]>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    Letter(char),
    Sequence(RulesRef),
    Alt(RulesRef, RulesRef),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input<'s> {
    rules: Rules,
    inputs: Vec<&'s str>,
}

fn matches_rule(rules: &Rules, rule_index: u32, input: &str) -> bool {
    fn visit_seq(
        rules: &Rules,
        seq: &[u32],
        input: &str,
        continuation: &mut dyn FnMut(&str) -> bool,
    ) -> bool {
        if let Some(&rule_index) = seq.first() {
            let remainder = &seq[1..];
            visit(rules, rule_index, input, &mut move |input| {
                visit_seq(rules, remainder, input, continuation)
            })
        } else {
            continuation(input)
        }
    }
    fn visit(
        rules: &Rules,
        rule_index: u32,
        input: &str,
        continuation: &mut dyn FnMut(&str) -> bool,
    ) -> bool {
        match &rules[&rule_index] {
            &Rule::Letter(c) => {
                if input.chars().next() == Some(c) {
                    continuation(&input[1..])
                } else {
                    false
                }
            }
            Rule::Sequence(seq) => visit_seq(rules, seq, input, continuation),
            Rule::Alt(a, b) => {
                if visit_seq(rules, a, input, continuation) {
                    true
                } else {
                    visit_seq(rules, b, input, continuation)
                }
            }
        }
    }
    visit(rules, rule_index, input, &mut |remainder| {
        remainder.len() == 0
    })
}

pub fn pt1(Input { rules, inputs }: &Input) -> usize {
    inputs
        .iter()
        .count_if(|input| matches_rule(rules, 0, input))
}

pub fn pt2(Input { rules, inputs }: &Input) -> usize {
    let mut rules = rules.clone();
    rules.insert(
        8,
        Rule::Alt(
            [42].iter().cloned().collect(),
            [42, 8].iter().cloned().collect(),
        ),
    );
    rules.insert(
        11,
        Rule::Alt(
            [42, 31].iter().cloned().collect(),
            [42, 11, 31].iter().cloned().collect(),
        ),
    );
    inputs
        .iter()
        .count_if(|input| matches_rule(&rules, 0, input))
}

pub fn parse(input: &str) -> Result<Input> {
    use framework::parser::*;
    let letter_rule = map(
        preceded(char('"'), terminated(anychar, char('"'))),
        |c: char| Rule::Letter(c),
    );
    fn space_take_u32(input: &str) -> IResult<u32> {
        preceded(char(' '), take_u32)(input)
    }
    fn sequence(input: &str) -> IResult<RulesRef> {
        map(
            pair(take_u32, opt(pair(space_take_u32, opt(space_take_u32)))),
            |(a, rem)| {
                let mut rules_ref = RulesRef::new();
                rules_ref.push(a);
                if let Some((b, rem)) = rem {
                    rules_ref.push(b);
                    if let Some(c) = rem {
                        rules_ref.push(c);
                    }
                }
                rules_ref
            },
        )(input)
    }
    let alt_or_sequence_rule = map(
        pair(sequence, opt(preceded(tag(" | "), sequence))),
        |(a, b)| {
            if let Some(b) = b {
                Rule::Alt(a, b)
            } else {
                Rule::Sequence(a)
            }
        },
    );
    let rule = pair(
        terminated(take_u32, tag(": ")),
        alt((letter_rule, alt_or_sequence_rule)),
    );
    let rules = map_opt(
        separated_list1(char('\n'), rule),
        |rules: Vec<(u32, Rule)>| {
            let mut output = Rules::with_capacity(rules.len());
            for (index, rule) in rules {
                match output.entry(index) {
                    Entry::Occupied(_) => return None,
                    Entry::Vacant(slot) => {
                        slot.insert(rule);
                    }
                }
            }
            Some(output)
        },
    );
    let inputs = separated_list1(char('\n'), alpha1);
    map(
        pair(rules, preceded(tag("\n\n"), inputs)),
        |(rules, inputs)| Input { rules, inputs },
    )(input)
    .into_result()
}

#[cfg(test)]
const COMMON_EXAMPLE: &str = "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

standard_tests!(
    parse []
    pt1 [
        "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb" => 2
        COMMON_EXAMPLE => 3
    ]
    pt2 [ COMMON_EXAMPLE => 12 ]
);
