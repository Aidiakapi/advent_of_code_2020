use std::unreachable;

use crate::prelude::*;

day!(23, parse => pt1, pt2);

const CUP_MASK: u64 = 0xfffffffff;

/// Uses bits 0..36 to represent each digit as a 4-bit sequence
/// It places the first bit on the right, and it's 0 based, so the
/// input 123456789 becomes: 0b1000_0111_0110_0101_0100_0011_0010_0001_0000
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cups(u64);

impl std::str::FromStr for Cups {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 9 {
            return Err(Error::InvalidInput("expected 9 digits"));
        }
        let mut seen_mask = 0u32;
        let mut cups = 0u64;
        for (index, char) in s.char_indices() {
            if !matches!(char, '1'..='9') {
                return Err(Error::InvalidInput("expected characters in range 1..=9"));
            }
            let digit = char as u8 - b'1';
            seen_mask |= 1u32 << digit;
            cups |= (digit as u64) << (index * 4);
        }

        if seen_mask != (1 << 9) - 1 {
            return Err(Error::InvalidInput(
                "expected each digit in range 1..=9 exactly once",
            ));
        }
        Ok(Cups(cups))
    }
}

impl std::fmt::Debug for Cups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cups({})", self)
    }
}

impl std::fmt::Display for Cups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars = [0; 9];
        for i in 0..9 {
            chars[i] = ((self.0 >> (i * 4)) & 0xf) as u8 + b'1';
        }
        f.write_str(unsafe { std::str::from_utf8_unchecked(&chars) })
    }
}

impl std::ops::Index<usize> for Cups {
    type Output = u8;

    #[rustfmt::skip]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 9);
        match (self.0 >> (index * 4)) & 0xf {
            0 => &0, 1 => &1, 2 => &2, 3 => &3, 4 => &4,
            5 => &5, 6 => &6, 7 => &7, 8 => &8,
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
impl Cups {
    fn rot_cw(self, mut steps: u8) -> Cups {
        steps = steps % 9;
        Cups((self.0 << (steps * 4) | self.0 >> (36 - steps * 4)) & CUP_MASK)
    }
    fn rot_ccw(self, steps: u8) -> Cups {
        self.rot_cw(9 - steps % 9)
    }
}

// Original implementation of pt1, which actually updated the entire cups with
// bitwise logic.
#[allow(dead_code)]
pub fn pt1_original(input: &Cups) -> Result<String> {
    let mut cups = *input;

    for _ in 0..100 {
        let n = cups[0];
        let dst = (4..9)
            .map(|i| (n + 9 - cups[i]) % 9)
            .position_min()
            .unwrap()
            + 1;

        let rotated = cups.rot_ccw(1).0;
        let removed = rotated >> 12;
        let before_insertion = removed & ((1 << (dst * 4)) - 1);
        let after_insertion = (removed & !((1 << (dst * 4)) - 1)) << 12;
        let insertion = (rotated & 0xfff) << (dst * 4);
        let result = before_insertion | insertion | after_insertion;
        cups = Cups(result);
    }

    cups = cups.rot_ccw((0..9).position(|i| cups[i] == 0).unwrap() as u8 + 1);
    let mut answer = cups.to_string();
    answer.pop();

    Ok(answer)
}

#[rustfmt::skip]
fn wrapping_dec<const COUNT: usize>(n: usize) -> usize {
    if n == 0 { COUNT - 1 } else { n - 1 }
}

fn pts<const COUNT: usize, const ITER: usize>(input: &Cups) -> Vec<usize> {
    assert!(COUNT >= 9);

    // Singly linked list linking from a cup number (same as index) to the next
    // cup's number going clockwise in the circle of cups.
    let mut links = Vec::with_capacity(COUNT);

    links.resize(9, 0);
    for i in 0..8 {
        links[input[i] as usize] = input[i + 1] as usize;
    }

    if COUNT == 9 {
        links[input[8] as usize] = input[0] as usize;
    } else {
        links[input[8] as usize] = 9;
        for i in 9..COUNT - 1 {
            links.push(i + 1);
        }
        links.push(input[0] as usize);
    }

    let mut current_cup = input[0] as usize;
    for _ in 0..ITER {
        // Next three clockwise cups
        let a = links[current_cup];
        let b = links[a];
        let c = links[b];

        // Destination
        let mut d = wrapping_dec::<COUNT>(current_cup);
        while d == a || d == b || d == c {
            d = wrapping_dec::<COUNT>(d);
        }

        // Move the three cups to its destination
        links[current_cup] = links[c];
        links[c] = links[d];
        links[d] = a;

        // Advance the current cup to the next one
        current_cup = links[current_cup];
    }

    links
}

fn pt1(input: &Cups) -> String {
    let links = pts::<9, 100>(input);
    let mut res = [0u8; 8];
    let mut curr = 0;
    for i in 0..8 {
        curr = links[curr];
        res[i] = curr as u8 + b'1';
    }
    std::str::from_utf8(&res).unwrap().to_owned()
}

pub fn pt2(input: &Cups) -> u64 {
    let links = pts::<1_000_000, 10_000_000>(input);
    let a = links[0];
    let b = links[a];
    (a as u64 + 1) * (b as u64 + 1)
}

pub fn parse(input: &str) -> Result<Cups> {
    input.parse()
}

standard_tests!(
    parse []
    pt1 [ "389125467" => "67384529" ]
    pt2 [ "389125467" => 149245887792 ]
);
