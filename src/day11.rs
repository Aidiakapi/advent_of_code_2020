use arrayvec::ArrayVec;

use crate::prelude::*;
day!(11, parse => pt1, pt2);

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    Floor,
    EmptySeat,
    FilledSeat,
}

fn get_visible_seats_pt1(
    cells: &[Vec<Cell>],
    size: isizex2,
    position: isizex2,
) -> ArrayVec<[usizex2; 8]> {
    position
        .neighbors()
        .iter()
        .cloned()
        .filter_map(|neighbor| {
            if neighbor.x >= 0
                && neighbor.x < size.x
                && neighbor.y >= 0
                && neighbor.y < size.y
                && cells[neighbor.y as usize][neighbor.x as usize] != Cell::Floor
            {
                neighbor.cast()
            } else {
                None
            }
        })
        .collect()
}

fn get_visible_seats_pt2(
    cells: &[Vec<Cell>],
    size: isizex2,
    position: isizex2,
) -> ArrayVec<[usizex2; 8]> {
    let mut result = ArrayVec::new();
    for &direction in isizex2::default().neighbors().iter() {
        let mut cell = position;
        loop {
            cell += direction;
            if cell.x < 0 || cell.x >= size.x || cell.y < 0 || cell.y >= size.y {
                break;
            }
            match cells[cell.y as usize][cell.x as usize] {
                Cell::Floor => {}
                _ => {
                    result.push(cell.cast().unwrap());
                    break;
                }
            }
        }
    }
    result
}

fn pts(
    input: &[Vec<Cell>],
    becomes_empty_at_threshold: usize,
    get_visible_seats: impl Fn(&[Vec<Cell>], isizex2, isizex2) -> ArrayVec<[usizex2; 8]>,
) -> Result<usize> {
    let mut layout = input.to_vec();

    // First step is accumulating which seats are visible from which other seats
    // any seat that cannot see at least the threshold amount of other seats
    // will never be able to flip to inactive, and can therefore be skipped
    // in its entirety.
    let mut vec2_storage = Vec::new();
    let mut ranges = Vec::new();
    let height = input.len() as isize;
    for (y, row) in input.iter().enumerate() {
        let width = row.len() as isize;
        for (x, cell) in row.iter().enumerate() {
            if *cell == Cell::Floor {
                continue;
            }
            layout[y][x] = Cell::FilledSeat;
            let visible_seats =
                get_visible_seats(&input, vec2!(width, height), vec2!(x as isize, y as isize));
            if visible_seats.len() < becomes_empty_at_threshold {
                continue;
            }
            let range = vec2_storage.len()..vec2_storage.len() + visible_seats.len();
            vec2_storage.extend_from_slice(&visible_seats);
            ranges.push((vec2!(x, y), range));
        }
    }
    let ranges = ranges
        .into_iter()
        .map(|(pos, range)| (pos, &vec2_storage[range]))
        .collect::<Vec<_>>();

    let mut swap_at = Vec::new();
    for _ in 0..1000 {
        for &(position, visible_seats) in &ranges {
            let visible_count = visible_seats
                .iter()
                .count_if(|pos| layout[pos.y][pos.x] == Cell::FilledSeat);
            let cell = layout[position.y][position.x];
            if (visible_count >= becomes_empty_at_threshold && cell == Cell::FilledSeat)
                || (visible_count == 0 && cell == Cell::EmptySeat)
            {
                swap_at.push(position);
            }
        }

        if swap_at.is_empty() {
            return Ok(layout
                .into_iter()
                .flat_map(Vec::into_iter)
                .count_if(|cell| cell == Cell::FilledSeat));
        }

        for position in &swap_at {
            let cell = &mut layout[position.y][position.x];
            *cell = if *cell == Cell::FilledSeat {
                Cell::EmptySeat
            } else {
                Cell::FilledSeat
            };
        }
        swap_at.clear();
    }

    Err(Error::NoSolution)
}

pub fn pt1(input: &[Vec<Cell>]) -> Result<usize> {
    pts(input, 4, get_visible_seats_pt1)
}

pub fn pt2(input: &[Vec<Cell>]) -> Result<usize> {
    pts(input, 5, get_visible_seats_pt2)
}

pub fn parse(input: &str) -> Result<Vec<Vec<Cell>>> {
    use framework::parser::*;
    let cell = map(one_of(".L"), |c: char| match c {
        '.' => Cell::Floor,
        'L' => Cell::EmptySeat,
        _ => unreachable!(),
    });
    separated_list1(char('\n'), many1(cell))(input).into_result()
}

#[cfg(test)]
const EXAMPLE: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

standard_tests!(
    parse []
    pt1 [ EXAMPLE => 37 ]
    pt2 [ EXAMPLE => 26 ]
);
