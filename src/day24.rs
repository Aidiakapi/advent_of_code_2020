use crate::prelude::*;

day!(24, parse_and_initialize => pt1, pt2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Direction {
    East = 0b000,
    West = 0b001,
    NorthEast = 0b010,
    SouthWest = 0b011,
    NorthWest = 0b100,
    SouthEast = 0b101,
}

impl Direction {
    #[rustfmt::skip]
    fn to_vec2(self) -> i32x2 {
        match self {
            Direction::East      => i32x2::new( 1,  0),
            Direction::West      => i32x2::new(-1,  0),
            Direction::NorthEast => i32x2::new( 0,  1),
            Direction::SouthWest => i32x2::new( 0, -1),
            Direction::NorthWest => i32x2::new(-1,  1),
            Direction::SouthEast => i32x2::new( 1, -1),
        }
    }

    #[inline]
    fn all() -> impl Iterator<Item = Direction> + Clone {
        (0u8..6).map(|n| unsafe { std::mem::transmute(n) })
    }
}

pub fn pt1(black_tiles: &HashSet<i32x2>) -> usize {
    black_tiles.len()
}

pub fn pt2(black_tiles: &HashSet<i32x2>) -> usize {
    let mut black_tiles = black_tiles.clone();
    let mut neighbor_count = HashMap::new();
    let mut pending_flips = Vec::new();

    for _ in 0..100 {
        for &tile in &black_tiles {
            neighbor_count.entry(tile).or_insert(0usize);
            for direction in Direction::all() {
                let neighbor = tile + direction.to_vec2();
                *neighbor_count.entry(neighbor).or_insert(0usize) += 1;
            }
        }

        for (&tile, &black_count) in &neighbor_count {
            if black_count == 2 {
                if !black_tiles.contains(&tile) {
                    pending_flips.push((tile, false));
                }
            // Condition might seem weird, but if black_count == 0, then it is
            // guaranteed to be black itself, otherwise it wouldn't have been
            // added to the collection in the first place.
            } else if black_count == 0 || (black_count > 2 && black_tiles.contains(&tile)) {
                pending_flips.push((tile, true));
            }
        }
        neighbor_count.clear();

        for &(tile, should_flip_to_white) in &pending_flips {
            if should_flip_to_white {
                black_tiles.remove(&tile);
            } else {
                black_tiles.insert(tile);
            }
        }
        pending_flips.clear();
    }

    black_tiles.len()
}

fn parse(input: &str) -> Result<Vec<Vec<Direction>>> {
    use framework::parser::*;
    let direction = alt((
        map(char('e'), |_| Direction::East),
        map(char('w'), |_| Direction::West),
        map(tag("ne"), |_| Direction::NorthEast),
        map(tag("sw"), |_| Direction::SouthWest),
        map(tag("nw"), |_| Direction::NorthWest),
        map(tag("se"), |_| Direction::SouthEast),
    ));
    separated_list1(char('\n'), many1(direction))(input).into_result()
}

pub fn parse_and_initialize(input: &str) -> Result<HashSet<i32x2>> {
    let instructions = parse(input)?;

    let mut black_tiles = HashSet::new();
    for instruction in &instructions {
        let mut pos = i32x2::default();
        for step in instruction {
            pos += step.to_vec2();
        }
        if !black_tiles.insert(pos) {
            black_tiles.remove(&pos);
        }
    }

    Ok(black_tiles)
}

#[cfg(test)]
const EXAMPLE: &str = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

standard_tests!(
    parse_and_initialize []
    pt1 [ EXAMPLE => 10 ]
    pt2 [ EXAMPLE => 2208 ]
);
