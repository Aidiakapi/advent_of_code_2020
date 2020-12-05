use crate::prelude::*;
day!(3, parse => pt1, pt2);

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Cell {
    Open,
    Tree,
}

pub fn count_trees_at_slope(input: &[Vec<Cell>], x_step: usize, y_step: usize) -> usize {
    let x = (0..).step_by(x_step);
    let y = input.iter().step_by(y_step);
    x.zip(y)
        .skip(1)
        .count_if(|(x, row)| row[x % row.len()] == Cell::Tree)
}

pub fn pt1(input: &[Vec<Cell>]) -> usize {
    count_trees_at_slope(input, 3, 1)
}

pub fn pt2(input: &[Vec<Cell>]) -> usize {
    let encountered = [
        count_trees_at_slope(input, 1, 1),
        count_trees_at_slope(input, 3, 1),
        count_trees_at_slope(input, 5, 1),
        count_trees_at_slope(input, 7, 1),
        count_trees_at_slope(input, 1, 2),
    ];
    encountered.iter().product()
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

standard_tests!(
    parse []
    pt1 [
"\
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
.#..#...#.#" => 7
    ]
    pt2 [
"\
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
.#..#...#.#" => 336]
);
