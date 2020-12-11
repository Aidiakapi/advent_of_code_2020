use crate::prelude::*;
day!(11, parse => pt1, pt2);

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    Floor,
    EmptySeat,
    FilledSeat,
}

fn count_occupied_seats_pt1(cells: &[Vec<Cell>], size: isizex2, position: isizex2) -> usize {
    position.neighbors().iter().count_if(|neighbor| {
        neighbor.x >= 0
            && neighbor.x < size.x
            && neighbor.y >= 0
            && neighbor.y < size.y
            && cells[neighbor.y as usize][neighbor.x as usize] == Cell::FilledSeat
    })
}

fn should_swap_pt1(occupied_seats: usize, is_empty: bool) -> bool {
    if is_empty {
        occupied_seats == 0
    } else {
        occupied_seats >= 4
    }
}

fn count_occupied_seats_pt2(cells: &[Vec<Cell>], size: isizex2, position: isizex2) -> usize {
    let mut count = 0;
    for &direction in isizex2::default().neighbors().iter() {
        let mut cell = position;
        loop {
            cell += direction;
            if cell.x < 0 || cell.x >= size.x || cell.y < 0 || cell.y >= size.y {
                break;
            }
            match cells[cell.y as usize][cell.x as usize] {
                Cell::Floor => {}
                Cell::EmptySeat => break,
                Cell::FilledSeat => {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

fn should_swap_pt2(occupied_seats: usize, is_empty: bool) -> bool {
    if is_empty {
        occupied_seats == 0
    } else {
        occupied_seats >= 5
    }
}

fn pts(
    input: &[Vec<Cell>],
    count_occupied_seats: impl Fn(&[Vec<Cell>], isizex2, isizex2) -> usize,
    should_swap: impl Fn(usize, bool) -> bool,
) -> Result<usize> {
    let mut current = input.to_vec();
    let mut backbuffer = current.clone();
    let height = current.len() as isize;
    for _ in 0..1_000 {
        std::mem::swap(&mut current, &mut backbuffer);
        let mut any_changes = false;
        for (y, row) in current.iter_mut().enumerate() {
            let width = row.len() as isize;
            for (x, cell) in row.iter_mut().enumerate() {
                if *cell == Cell::Floor {
                    continue;
                }
                let occupied_seats = count_occupied_seats(
                    &backbuffer,
                    vec2!(width, height),
                    vec2!(x as isize, y as isize),
                );
                *cell = backbuffer[y][x];
                if should_swap(occupied_seats, *cell == Cell::EmptySeat) {
                    any_changes = true;
                    *cell = if *cell == Cell::EmptySeat {
                        Cell::FilledSeat
                    } else {
                        Cell::EmptySeat
                    };
                }
            }
        }
        if !any_changes {
            return Ok(current
                .into_iter()
                .flat_map(Vec::into_iter)
                .count_if(|cell| cell == Cell::FilledSeat));
        }
    }
    Err(Error::NoSolution)
}

pub fn pt1(input: &[Vec<Cell>]) -> Result<usize> {
    pts(input, count_occupied_seats_pt1, should_swap_pt1)
}

pub fn pt2(input: &[Vec<Cell>]) -> Result<usize> {
    pts(input, count_occupied_seats_pt2, should_swap_pt2)
}

pub fn parse(input: &str) -> Result<Vec<Vec<Cell>>> {
    use framework::parser::*;
    let cell = map(alt((char('.'), char('L'), char('#'))), |c: char| match c {
        '.' => Cell::Floor,
        'L' => Cell::EmptySeat,
        '#' => Cell::FilledSeat,
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
