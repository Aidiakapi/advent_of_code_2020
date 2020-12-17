use crate::prelude::*;

day!(17, parse => pt1, pt2);

pub fn pt1(cells: &HashSet<i32x2>) -> usize {
    let mut cells = cells.into_iter().map(|cell| cell.with_z(0)).collect();
    let mut prev_cells = HashSet::new();
    let mut accumulator = HashMap::<i32x3, usize>::new();
    for _ in 0..6 {
        std::mem::swap(&mut cells, &mut prev_cells);
        for &cell in &prev_cells {
            for x in -1..2 {
                for y in -1..2 {
                    for z in -1..2 {
                        let neighbor = cell + vec3!(x, y, z);
                        *accumulator.entry(neighbor).or_default() += 1;
                    }
                }
            }
            // Subtract one because the current cell was added as a neighbor
            *accumulator.get_mut(&cell).unwrap() -= 1;
        }
        for (&cell, &neighbor_count) in &accumulator {
            if (neighbor_count == 2 || neighbor_count == 3)
                && (neighbor_count == 3 || prev_cells.contains(&cell))
            {
                cells.insert(cell);
            }
        }
        accumulator.clear();
        prev_cells.clear();
    }

    cells.len()
}

pub fn pt2(cells: &HashSet<i32x2>) -> usize {
    type Cell = (i32, i32, i32, i32);
    let mut cells = cells
        .into_iter()
        .map(|cell| (cell.x, cell.y, 0, 0))
        .collect();
    let mut prev_cells = HashSet::new();
    let mut accumulator = HashMap::<Cell, usize>::new();
    for _ in 0..6 {
        std::mem::swap(&mut cells, &mut prev_cells);
        for &cell in &prev_cells {
            for x in -1..2 {
                let x = cell.0 + x;
                for y in -1..2 {
                    let y = cell.1 + y;
                    for z in -1..2 {
                        let z = cell.2 + z;
                        for w in -1..2 {
                            let w = cell.3 + w;
                            let neighbor = (x, y, z, w);
                            *accumulator.entry(neighbor).or_default() += 1;
                        }
                    }
                }
            }
            // Subtract one because the current cell was added as a neighbor
            *accumulator.get_mut(&cell).unwrap() -= 1;
        }
        for (&cell, &neighbor_count) in &accumulator {
            if (neighbor_count == 2 || neighbor_count == 3)
                && (neighbor_count == 3 || prev_cells.contains(&cell))
            {
                cells.insert(cell);
            }
        }
        accumulator.clear();
        prev_cells.clear();
    }

    cells.len()
}

pub fn parse(input: &str) -> Result<HashSet<i32x2>> {
    use framework::parser::*;
    map(
        fold_many1(
            one_of(".#\n"),
            (0, 0, HashSet::new()),
            |(x, y, mut set), c| match c {
                '.' => (x + 1, y, set),
                '#' => {
                    set.insert(vec2!(x, y));
                    (x + 1, y, set)
                }
                '\n' => (0, y + 1, set),
                _ => unreachable!(),
            },
        ),
        |(_, _, set)| set,
    )(input)
    .into_result()
}

#[cfg(test)]
const EXAMPLE: &str = "\
.#.
..#
###";

standard_tests!(
    parse []
    pt1 [ EXAMPLE => 112 ]
    pt2 [ EXAMPLE => 848 ]
);
