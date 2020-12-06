use crate::prelude::*;
day!(3, parse => pt1, pt2);

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    Open,
    Tree,
}

pub fn count_trees_at_slope(input: &[Vec<Cell>], step: usizex2) -> usize {
    let x = (0..).step_by(step.x);
    let y = input.iter().step_by(step.y);
    x.zip(y)
        .skip(1)
        .count_if(|(x, row)| row[x % row.len()] == Cell::Tree)
}

pub fn pt1(input: &[Vec<Cell>]) -> usize {
    count_trees_at_slope(input, vec2!(3, 1))
}

pub fn pt2(input: &[Vec<Cell>]) -> usize {
    let slopes = [
        vec2!(1, 1),
        vec2!(3, 1),
        vec2!(5, 1),
        vec2!(7, 1),
        vec2!(1, 2),
    ];
    slopes
        .iter()
        .map(|&step| count_trees_at_slope(input, step))
        .product()
}

pub fn parse(input: &astr) -> Result<Vec<Vec<Cell>>> {
    use framework::parser::*;
    fn cell(input: &astr) -> IResult<Cell> {
        alt((
            map(char(achar::Dot), |_| Cell::Open),
            map(char(achar::Hash), |_| Cell::Tree),
        ))(input)
    }
    separated_list1(char(achar::LineFeed), many1(cell))(input).into_result()
}

#[cfg(test)]
const EXAMPLE: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

standard_tests!(
    parse []
    pt1 [ EXAMPLE => 7 ]
    pt2 [ EXAMPLE => 336 ]
);
