use crate::prelude::*;
day!(13, parse => pt1, pt2);

pub fn pt1((current_time, bus_lines): &(u64, Vec<Option<u64>>)) -> Result<u64> {
    bus_lines
        .iter()
        .cloned()
        .filter_map(|x| x)
        .map(|bus_line| (bus_line, current_time + bus_line - current_time % bus_line))
        .min_by_key(|&(_, next_departure)| next_departure)
        .map(|(bus_line, next_departure)| bus_line * (next_departure - current_time))
        .ok_or(Error::NoSolution)
}

/// Represents all positive integers where `t % modulo = offset`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Set {
    modulo: u64,
    offset: u64,
}

impl Default for Set {
    fn default() -> Self {
        Set {
            modulo: 1,
            offset: 0,
        }
    }
}

impl std::fmt::Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "t % {} = {}", self.modulo, self.offset)
    }
}

/// Computes the intersection for the set.
/// I have no idea what the time complexity is of finding the offset, but I'm
/// sure there's some mathematically way to compute it in constant time. However
/// this is basically instant in practice.
fn intersect_sets(a: Set, b: Set) -> Set {
    let modulo = num::integer::lcm(a.modulo, b.modulo);
    let mut a_value = a.offset;
    let mut b_value = b.offset;
    loop {
        match a_value.cmp(&b_value) {
            std::cmp::Ordering::Less => {
                a_value += a.modulo * ((b_value - a_value) / a.modulo).max(1)
            }
            std::cmp::Ordering::Greater => {
                b_value += b.modulo * ((a_value - b_value) / b.modulo).max(1)
            }
            std::cmp::Ordering::Equal => break,
        }
    }
    Set {
        modulo,
        offset: a_value,
    }
}

pub fn pt2((_, bus_lines): &(u64, Vec<Option<u64>>)) -> u64 {
    // Lets say the input lines are:
    // 2, 3, x, 7
    // Then the output `t` must satisfy these constraints:
    // (t + 0) % 2 = 0
    // (t + 1) % 3 = 0
    // (t + 3) % 7 = 0
    //
    // This can be simplified to this:
    // t % 2 = 0
    // t % 3 = 2
    // t % 7 = 4
    //
    // These each map to a sequence of integer solutions
    //   a:   0,   2,   4,   6,   8,  10,  12,  14,  16,  18, ... (t %  2 =  0)
    //   b:   2,   5,   8,  11,  14,  17,  20,  23,  26,  29, ... (t %  3 =  2)
    //   c:   4,  11,  18,  25,  32,  39,  46,  53,  60,  67, ... (t %  7 =  4)
    //
    // The intersection between these sequences becomes:
    //  ab:   2,   8,  14,  20,  26,  32,  38,  44,  50,  56, ... (t %  6 =  2)
    //  ac:   4,  18,  32,  46,  60,  74,  88, 102, 116, 130, ... (t % 14 =  4)
    //  bc:  11,  32,  53,  74,  95, 116, 137, 158, 179, 200, ... (t % 21 = 11)
    //
    // Our output is the intersection of all three
    // abc:  32,  74, 116, 158, 200, 242, 284, 326, 368, 410, ... (t % 42 = 32)
    bus_lines
        .iter()
        .cloned()
        .enumerate()
        .filter_map(|(i, bus_line)| {
            if let Some(bus_line) = bus_line {
                Some(Set {
                    modulo: bus_line,
                    offset: bus_line - (i as u64 % bus_line),
                })
            } else {
                None
            }
        })
        .fold(Set::default(), |acc, value| intersect_sets(acc, value))
        .offset
}

pub fn parse(input: &str) -> Result<(u64, Vec<Option<u64>>)> {
    use framework::parser::*;
    pair(
        terminated(take_u64, char('\n')),
        separated_list1(
            char(','),
            alt((map(char('x'), |_| None), map(take_u64, Some))),
        ),
    )(input)
    .into_result()
}

#[cfg(test)]
const EXAMPLE: &str = "\
939
7,13,x,x,59,x,31,19";

standard_tests!(
    parse []
    pt1 [ EXAMPLE => 295 ]
    pt2 [
        EXAMPLE => 1068781
        "0\n67,7,59,61" => 754018
        "0\n67,x,7,59,61" => 779210
        "0\n67,7,x,59,61" => 1261476
        "0\n1789,37,47,1889" => 1202161486
    ]
);
