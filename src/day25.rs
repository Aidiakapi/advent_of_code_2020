use crate::prelude::*;

day!(25, parse => pt1, pt2);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Transformer {
    current: u64,
    subject_number: u64,
}

impl Default for Transformer {
    fn default() -> Self {
        Transformer {
            current: 1,
            subject_number: 7,
        }
    }
}

impl Iterator for Transformer {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.current = (self.current * self.subject_number) % 20201227;
        Some(self.current)
    }
}

pub fn pt1(&(mut card, mut door): &(u64, u64)) -> u64 {
    let mut transformer = Transformer::default().enumerate();
    loop {
        let (_, nr) = transformer.next().unwrap();

        if nr == card || nr == door {
            if nr == door {
                std::mem::swap(&mut card, &mut door);
            }
            break;
        }
    }
    let skip_count = loop {
        let (count, nr) = transformer.next().unwrap();
        if nr == door {
            break count;
        }
    };

    Transformer {
        current: 1,
        subject_number: card,
    }
    .skip(skip_count)
    .next()
    .unwrap()
}

pub fn pt2(_: &(u64, u64)) -> &'static str {
    "victory"
}

pub fn parse(input: &str) -> Result<(u64, u64)> {
    use framework::parser::*;
    pair(take_u64, preceded(char('\n'), take_u64))(input).into_result()
}

standard_tests!(
    parse []
    pt1 [ "5764801\n17807724" => 14897079 ]
    pt2 []
);
