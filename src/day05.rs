use crate::prelude::*;
day!(5, parse => pt1, pt2);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Partition {
    Low,
    High,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SeatCode {
    row: [Partition; 7],
    column: [Partition; 3],
}

fn binary_partition(partitions: &[Partition]) -> u32 {
    assert!(partitions.len() <= 32);
    let mut result = 0u32;
    let mut current_bit = 1 << (partitions.len() - 1);
    for &partition in partitions.iter() {
        result |= current_bit & (-(partition as i32) as u32);
        current_bit >>= 1;
    }
    debug_assert!(current_bit == 0);
    result
}

impl SeatCode {
    fn to_row_and_column(&self) -> (u32, u32) {
        (binary_partition(&self.row), binary_partition(&self.column))
    }

    fn to_id(&self) -> u32 {
        let (row, column) = self.to_row_and_column();
        row * 8 + column
    }
}

pub fn pt1(input: &[SeatCode]) -> Result<u32> {
    input
        .iter()
        .map(SeatCode::to_id)
        .max()
        .ok_or(Error::NoSolution)
}

pub fn pt2(input: &[SeatCode]) -> Result<u32> {
    let mut seat_ids = input.iter().map(SeatCode::to_id).collect::<Vec<_>>();
    seat_ids.sort_unstable();
    seat_ids
        .windows(2)
        .find(|window| window[1] - window[0] == 2)
        .map(|window| window[0] + 1)
        .ok_or(Error::NoSolution)
}

pub fn parse(input: &astr) -> Result<Vec<SeatCode>> {
    use framework::parser::*;
    fn row(input: &astr) -> IResult<Partition> {
        alt((
            map(char(achar::F), |_| Partition::Low),
            map(char(achar::B), |_| Partition::High),
        ))(input)
    }
    fn column(input: &astr) -> IResult<Partition> {
        alt((
            map(char(achar::L), |_| Partition::Low),
            map(char(achar::R), |_| Partition::High),
        ))(input)
    }

    let rows = map(
        tuple((row, row, row, row, row, row, row)),
        |(a, b, c, d, e, f, g)| [a, b, c, d, e, f, g],
    );
    let columns = map(tuple((column, column, column)), |(a, b, c)| [a, b, c]);
    let seat_code = map(pair(rows, columns), |(row, column)| SeatCode {
        row,
        column,
    });
    separated_list1(char(achar::LineFeed), seat_code)(input).into_result()
}

standard_tests!(
    parse [ "BFFFBBFRLL" => vec![
        SeatCode {
            row: [
                Partition::High,
                Partition::Low,
                Partition::Low,
                Partition::Low,
                Partition::High,
                Partition::High,
                Partition::Low,
            ],
            column: [
                Partition::High,
                Partition::Low,
                Partition::Low,
            ],
        }
    ]]
    pt1 [ "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL" => 820 ]
);
