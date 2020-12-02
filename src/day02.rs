use crate::prelude::*;
day!(2, parse => pt1, pt2);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordPolicy {
    minimum: usize, // pt2: first  1-based password index
    maximum: usize, // pt2: second 1-based password index
    character: u8,
}

type Input<'a> = (PasswordPolicy, &'a [u8]);

fn is_valid_pt1((policy, input): &Input) -> bool {
    let count = input.iter().count_if(|&x| x == policy.character);
    policy.minimum <= count && count <= policy.maximum
}

fn is_valid_pt2((policy, input): &Input) -> Result<bool> {
    let at2 = *input
        .get(policy.maximum.overflowing_sub(1).0)
        .ok_or(Error::InvalidInput("index out of range"))?;
    let at1 = *input
        .get(policy.minimum.overflowing_sub(1).0)
        .ok_or(Error::InvalidInput("index out of range"))?;
    Ok((at1 == policy.character) != (at2 == policy.character))
}

pub fn pt1(input: &[Input]) -> usize {
    input.into_iter().count_if(is_valid_pt1)
}

pub fn pt2(input: &[Input]) -> Result<usize> {
    input.into_iter().count_if_res(is_valid_pt2)
}

pub fn parse(input: &[u8]) -> Result<Vec<Input>> {
    use framework::parser::*;
    let password_policy = map(
        tuple((
            terminated(take_usize, tag(b"-")),
            terminated(take_usize, tag(b" ")),
            terminated(take(1usize), tag(b": ")),
        )),
        |(minimum, maximum, character)| PasswordPolicy {
            minimum,
            maximum,
            character: character[0],
        },
    );
    let row = pair(password_policy, take_while(alpha));
    separated_list1(newline, row)(input).into_result()
}

#[cfg(test)]
const EXAMPLE: &'static [u8] = b"\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

standard_tests!(
    parse [ EXAMPLE => vec![
        (PasswordPolicy { minimum: 1, maximum: 3, character: b'a' }, b"abcde" as &[u8]),
        (PasswordPolicy { minimum: 1, maximum: 3, character: b'b' }, b"cdefg" as &[u8]),
        (PasswordPolicy { minimum: 2, maximum: 9, character: b'c' }, b"ccccccccc" as &[u8]),
    ] ]
    pt1 [ EXAMPLE => 2 ]
    pt2 [ EXAMPLE => 1 ]
);
