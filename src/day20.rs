use crate::prelude::*;
use arrayvec::ArrayVec;
use bitvec::prelude::*;
use std::{cell::Cell, collections::hash_map::Entry, num::NonZeroUsize};

day!(20, parse => pt1, pt2);

type Image = u128; // 10x10 image with 100 bits set to 0 or 1

#[derive(Clone, PartialEq, Eq)]
pub struct Tile {
    id: u32,
    image: Image,
}

// fn visualize<const N: usize>(image: Image) -> String {
//     let mut visualization = String::with_capacity(N * N + N - 1);
//     for y in 0..N {
//         if y != 0 {
//             visualization.push('\n');
//         }
//         for x in 0..N {
//             visualization.push(if (image >> (y * N + x)) & 1 == 1 {
//                 '#'
//             } else {
//                 '.'
//             });
//         }
//     }
//     visualization
// }

const fn index(x: usize, y: usize) -> usize {
    if x >= 10 || y >= 10 {
        panic!("index out of range");
    }
    x + y * 10
}

fn transform(
    image: Image,
    mut transformation: impl FnMut(usize, usize) -> (usize, usize),
) -> Image {
    let mut res = 0;
    for y in 0..10 {
        for x in 0..10 {
            let (nx, ny) = transformation(x, y);
            res |= ((image >> index(x, y)) & 1) << index(nx, ny);
        }
    }
    res
}

fn rotate_90(image: Image) -> Image {
    transform(image, |x, y| (9 - y, x))
}

const fn rotate_180(image: Image) -> Image {
    image.reverse_bits() >> (128 - 100)
}

fn rotate_270(image: Image) -> Image {
    transform(image, |x, y| (y, 9 - x))
}

fn flip_x(image: Image) -> Image {
    transform(image, |x, y| (9 - x, y))
}

fn flip_y(image: Image) -> Image {
    transform(image, |x, y| (x, 9 - y))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

const fn get_all_borders(image: Image) -> [u16; 4] {
    [
        get_border(image, Direction::Top),
        get_border(image, Direction::Right),
        get_border(image, Direction::Bottom),
        get_border(image, Direction::Left),
    ]
}

#[rustfmt::skip]
#[inline]
const fn get_border(image: Image, direction: Direction) -> u16 {
    const BITS_10: u128 = 0b1111111111;
    match direction {
        Direction::Top    => ((image & (BITS_10 << 00)) >> 00) as u16,
        Direction::Bottom => ((image & (BITS_10 << 90)) >> 90) as u16,
        Direction::Left   => (
                                 (image >> (00 - 0) & 0x001) |
                                 (image >> (10 - 1) & 0x002) |
                                 (image >> (20 - 2) & 0x004) |
                                 (image >> (30 - 3) & 0x008) |
                                 (image >> (40 - 4) & 0x010) |
                                 (image >> (50 - 5) & 0x020) |
                                 (image >> (60 - 6) & 0x040) |
                                 (image >> (70 - 7) & 0x080) |
                                 (image >> (80 - 8) & 0x100) |
                                 (image >> (90 - 9) & 0x200)
                             ) as u16,
        Direction::Right  => (
                                 (image >> (09 - 0) & 0x001) |
                                 (image >> (19 - 1) & 0x002) |
                                 (image >> (29 - 2) & 0x004) |
                                 (image >> (39 - 3) & 0x008) |
                                 (image >> (49 - 4) & 0x010) |
                                 (image >> (59 - 5) & 0x020) |
                                 (image >> (69 - 6) & 0x040) |
                                 (image >> (79 - 7) & 0x080) |
                                 (image >> (89 - 8) & 0x100) |
                                 (image >> (99 - 9) & 0x200)
                             ) as u16,
    }
}

impl Direction {
    #[inline]
    const fn rotate_90(self) -> Direction {
        unsafe { std::mem::transmute((self as u8 + 1) % 4) }
    }

    #[inline]
    const fn rotate_180(self) -> Direction {
        unsafe { std::mem::transmute((self as u8 + 2) % 4) }
    }

    #[inline]
    const fn rotate_270(self) -> Direction {
        unsafe { std::mem::transmute((self as u8 + 3) % 4) }
    }

    #[inline]
    const fn flip_x(self) -> Direction {
        if self as u8 & 1 == 1 {
            self.rotate_180()
        } else {
            self
        }
    }

    #[inline]
    const fn flip_y(self) -> Direction {
        if self as u8 & 1 == 0 {
            self.rotate_180()
        } else {
            self
        }
    }
}

const fn get_dual(border: u16) -> u16 {
    border.reverse_bits() >> (16 - 10)
}

// Value is one (no connection) or two (when there's a connection) sets of a
// tile index and the border direction.
type MatchingBorders = HashMap<u16, ((usize, Direction), Option<(usize, Direction)>)>;
fn get_matching_borders(tiles: &[Tile]) -> Result<MatchingBorders> {
    let mut matching_borders = MatchingBorders::new();
    for (i, tile) in tiles.iter().enumerate() {
        for (direction, &border) in get_all_borders(tile.image).iter().enumerate() {
            // Should've done something like TryFrom, but whatever...
            let direction: Direction = unsafe { std::mem::transmute(direction as u8) };
            // There is a dual to each border for when it's flipped, use the
            // numerically smallest one to always have a consistent orientation.
            let dual = get_dual(border);
            if dual == border {
                return Err(Error::InvalidInput("Border's dual may not match itself"));
            }
            let border = border.min(dual);
            match matching_borders.entry(border) {
                Entry::Occupied(mut s) => {
                    let (_, second) = s.get_mut();
                    if second.is_some() {
                        return Err(Error::NoSolution);
                    }
                    *second = Some((i, direction));
                }
                Entry::Vacant(s) => {
                    s.insert(((i, direction), None));
                }
            }
        }
    }
    Ok(matching_borders)
}

pub fn pt1(tiles: &[Tile]) -> Result<u64> {
    let matching_borders = get_matching_borders(tiles)?;
    let mut shared_border_count = vec![0; tiles.len()];
    for (_, &((a, _), b)) in &matching_borders {
        if let Some((b, _)) = b {
            shared_border_count[a] += 1;
            shared_border_count[b] += 1;
        }
    }
    let (count, product) = shared_border_count
        .iter()
        .cloned()
        .enumerate()
        .filter(|&(_, border_count)| border_count == 2)
        .fold((0, 1), |(count, product), (tile_index, _)| {
            (count + 1, product * (tiles[tile_index].id as u64))
        });
    if count == 4 {
        Ok(product)
    } else {
        Err(Error::NoSolution)
    }
}

pub fn pt2(tiles: &[Tile]) -> Result<usize> {
    let tile_size = num::integer::sqrt(tiles.len());
    if tile_size * tile_size != tiles.len() {
        return Err(Error::InvalidInput("Number of tiles isn't a square."));
    }

    let matching_borders = get_matching_borders(tiles)?;

    #[derive(Clone, Copy, PartialEq, Eq)]
    struct Connection {
        to_index: usize,
        from_direction: Direction,
        to_direction: Direction,
    }
    type Connectivity = ArrayVec<[Connection; 4]>;
    let mut connectivity = vec![Connectivity::new(); tiles.len()];
    for (_, &((a_idx, a_dir), b)) in &matching_borders {
        if let Some((b_idx, b_dir)) = b {
            connectivity[a_idx].push(Connection {
                to_index: b_idx,
                from_direction: a_dir,
                to_direction: b_dir,
            });
            connectivity[b_idx].push(Connection {
                to_index: a_idx,
                from_direction: b_dir,
                to_direction: a_dir,
            });
        }
    }
    for connections in &mut connectivity {
        connections.sort_unstable_by_key(|connection| connection.from_direction);
    }

    fn update_connectivity(
        connectivity: &mut Vec<Connectivity>,
        tile_index: usize,
        mut transformation: impl FnMut(Direction) -> Direction,
    ) {
        for i in 0..connectivity[tile_index].len() {
            let forward = &mut connectivity[tile_index][i];
            let old_dir = forward.from_direction;
            let new_dir = transformation(old_dir);
            forward.from_direction = new_dir;
            let to_index = forward.to_index;
            for backward in &mut connectivity[to_index] {
                if backward.to_index == tile_index {
                    assert_eq!(old_dir, backward.to_direction);
                    backward.to_direction = new_dir;
                    break;
                }
            }
        }
    }

    // The strategy for assembling the pieces is pretty straightforward:
    // 1. Start with any corner, rotate it so its two connecting edges are facing
    //    to the right and down.
    // 2. Expand from the bottom edge, and continually look at the opposite edge.
    // 3. Once we hit another corner piece, the left hand side has been resolved.
    // 4. For each piece, continually expand rightwards, following the same strategy.
    let (starting_tile, starting_connectivity) = connectivity
        .iter()
        .enumerate()
        .find(|(_, connectivity)| connectivity.len() == 2)
        .ok_or(Error::NoSolution)?;

    let mut images = tiles.iter().map(|tile| tile.image).collect::<Vec<_>>();

    // Orient the starting image into the right direction
    match (
        starting_connectivity[0].from_direction,
        starting_connectivity[1].from_direction,
    ) {
        (Direction::Right, Direction::Bottom) => {
            // No rotation
        }
        (Direction::Top, Direction::Right) => {
            images[starting_tile] = rotate_90(images[starting_tile]);
            update_connectivity(&mut connectivity, starting_tile, Direction::rotate_90);
        }
        (Direction::Top, Direction::Left) => {
            images[starting_tile] = rotate_180(images[starting_tile]);
            update_connectivity(&mut connectivity, starting_tile, Direction::rotate_180);
        }
        (Direction::Bottom, Direction::Left) => {
            images[starting_tile] = rotate_270(images[starting_tile]);
            update_connectivity(&mut connectivity, starting_tile, Direction::rotate_270);
        }
        _ => return Err(Error::NoSolution),
    }

    let mut assembly = Vec::with_capacity(tiles.len());
    assembly.push(starting_tile);

    // Expand vertically
    for y in 1..tile_size {
        let tile_above = assembly[y - 1];
        let connection = *connectivity[tile_above]
            .iter()
            .find(|connection| connection.from_direction == Direction::Bottom)
            .ok_or(Error::NoSolution)?;
        // Rotate to align properly
        let image = &mut images[connection.to_index];
        match connection.to_direction {
            Direction::Top => {
                // Nothing to do
            }
            Direction::Left => {
                *image = rotate_90(*image);
                update_connectivity(&mut connectivity, connection.to_index, Direction::rotate_90);
            }
            Direction::Bottom => {
                *image = rotate_180(*image);
                update_connectivity(
                    &mut connectivity,
                    connection.to_index,
                    Direction::rotate_180,
                );
            }
            Direction::Right => {
                *image = rotate_270(*image);
                update_connectivity(
                    &mut connectivity,
                    connection.to_index,
                    Direction::rotate_270,
                );
            }
        }
        // Check if it has to flip
        let top_border = get_border(*image, Direction::Top);
        let bottom_border = get_border(images[tile_above], Direction::Bottom);
        if bottom_border != top_border {
            debug_assert_eq!(bottom_border, get_dual(top_border));
            let image = &mut images[connection.to_index];
            *image = flip_x(*image);
            update_connectivity(&mut connectivity, connection.to_index, Direction::flip_x);
        }

        assembly.push(connection.to_index);
    }

    for x in 1..tile_size {
        for y in 0..tile_size {
            let tile_left = assembly[y + x * tile_size - tile_size];
            let connection = *connectivity[tile_left]
                .iter()
                .find(|connection| connection.from_direction == Direction::Right)
                .ok_or(Error::NoSolution)?;
            // Rotate to align properly
            let image = &mut images[connection.to_index];
            match connection.to_direction {
                Direction::Left => {
                    // Nothing to do
                }
                Direction::Bottom => {
                    *image = rotate_90(*image);
                    update_connectivity(
                        &mut connectivity,
                        connection.to_index,
                        Direction::rotate_90,
                    );
                }
                Direction::Right => {
                    *image = rotate_180(*image);
                    update_connectivity(
                        &mut connectivity,
                        connection.to_index,
                        Direction::rotate_180,
                    );
                }
                Direction::Top => {
                    *image = rotate_270(*image);
                    update_connectivity(
                        &mut connectivity,
                        connection.to_index,
                        Direction::rotate_270,
                    );
                }
            }
            // Check if it has to flip
            let left_border = get_border(*image, Direction::Left);
            let right_border = get_border(images[tile_left], Direction::Right);
            if right_border != left_border {
                debug_assert_eq!(right_border, get_dual(left_border));
                let image = &mut images[connection.to_index];
                *image = flip_y(*image);
                update_connectivity(&mut connectivity, connection.to_index, Direction::flip_y);
            }

            assembly.push(connection.to_index);
        }
    }

    // Trim the border from the images
    let images = images
        .into_iter()
        .map(|src| {
            const BITS_8: u128 = 0b11111111;
            (00 | ((src >> 11 & BITS_8) << 0x00)
                | ((src >> 21 & BITS_8) << 0x08)
                | ((src >> 31 & BITS_8) << 0x10)
                | ((src >> 41 & BITS_8) << 0x18)
                | ((src >> 51 & BITS_8) << 0x20)
                | ((src >> 61 & BITS_8) << 0x28)
                | ((src >> 71 & BITS_8) << 0x30)
                | ((src >> 81 & BITS_8) << 0x38)) as u64
        })
        .collect::<Vec<_>>();

    let big_image_size = tile_size * 8;
    let mut big_image = bitvec![0; big_image_size * big_image_size];

    for y in 0..big_image_size {
        for x in 0..big_image_size {
            let image = images[assembly[(y / 8) + (x / 8) * tile_size]];
            let is_set = (image >> ((y % 8) * 8 + x % 8)) & 1 == 1;
            big_image.set(y * big_image_size + x, is_set);
        }
    }

    // Find sea monsters
    const SEA_MONSTER: &str = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ";
    let mut sea_monster_positions = SEA_MONSTER
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| usizex2 { x, y })
        })
        .collect::<Vec<_>>();
    let mut sea_monster_size = vec2!(
        sea_monster_positions.iter().map(|p| p.x).max().unwrap() + 1,
        sea_monster_positions.iter().map(|p| p.y).max().unwrap() + 1
    );

    fn find_sea_monsters(
        sea_monster_size: usizex2,
        sea_monster_positions: &[usizex2],
        big_image: &BitVec,
        big_image_size: usize,
    ) -> Option<NonZeroUsize> {
        let mut matches = 0;
        for x in 0..big_image_size - sea_monster_size.x {
            'outer: for y in 0..big_image_size - sea_monster_size.y {
                for &pos in sea_monster_positions {
                    if !big_image[(y + pos.y) * big_image_size + x + pos.x] {
                        continue 'outer;
                    }
                }
                matches += 1;
            }
        }
        NonZeroUsize::new(matches)
    }
    fn flip_sea_monster(sea_monster_size: &mut usizex2, sea_monster_positions: &mut [usizex2]) {
        for pos in sea_monster_positions {
            *pos = vec2!(sea_monster_size.x - 1 - pos.x, pos.y);
        }
    }
    fn rotate_sea_monster(sea_monster_size: &mut usizex2, sea_monster_positions: &mut [usizex2]) {
        for pos in sea_monster_positions {
            *pos = vec2!(sea_monster_size.y - 1 - pos.y, pos.x);
        }
        *sea_monster_size = vec2!(sea_monster_size.y, sea_monster_size.x);
    }

    let should_flip = Cell::new(false);
    let mut find_or_transform = || {
        if let Some(result) = find_sea_monsters(
            sea_monster_size,
            &sea_monster_positions,
            &big_image,
            big_image_size,
        ) {
            Some(result)
        } else if should_flip.get() {
            flip_sea_monster(&mut sea_monster_size, &mut sea_monster_positions);
            None
        } else {
            rotate_sea_monster(&mut sea_monster_size, &mut sea_monster_positions);
            None
        }
    };

    find_or_transform()
        .or_else(&mut find_or_transform)
        .or_else(&mut find_or_transform)
        .or_else(&mut find_or_transform)
        .or_else(|| {
            should_flip.set(true);
            let result = find_or_transform();
            should_flip.set(false);
            result
        })
        .or_else(&mut find_or_transform)
        .or_else(&mut find_or_transform)
        .or_else(&mut find_or_transform)
        .map(|x| big_image.count_ones() - sea_monster_positions.len() * x.get())
        .ok_or(Error::NoSolution)
}

pub fn parse(input: &str) -> Result<Vec<Tile>> {
    use framework::parser::*;
    let input = input.trim_end();
    let tile_id = preceded(tag("Tile "), terminated(take_u32, char(':')));
    let row = preceded(
        char('\n'),
        fold_many_m_n(10, 10, one_of(".#"), 0, |acc, n| {
            (acc << 1) | if n == '#' { 1 } else { 0 }
        }),
    );
    let image = map(
        fold_many_m_n(10, 10, row, 0, |acc, n| (acc << 10) | n),
        rotate_180,
    );
    let tile = map(pair(tile_id, image), |(id, image)| Tile { id, image });
    separated_list1(tag("\n\n"), tile)(input).into_result()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    standard_tests!(
        !no_module
        parse []
        pt1 [ EXAMPLE => 20899048083289 ]
        pt2 [ EXAMPLE => 273 ]
    );

    #[test]
    fn direction_flips() {
        assert_eq!(Direction::Left, Direction::Right.flip_x());
        assert_eq!(Direction::Right, Direction::Left.flip_x());
        assert_eq!(Direction::Left, Direction::Left.flip_y());
        assert_eq!(Direction::Right, Direction::Right.flip_y());

        assert_eq!(Direction::Top, Direction::Bottom.flip_y());
        assert_eq!(Direction::Bottom, Direction::Top.flip_y());
        assert_eq!(Direction::Top, Direction::Top.flip_x());
        assert_eq!(Direction::Bottom, Direction::Bottom.flip_x());
    }
}
