use crate::prelude::*;
day!(12, parse => pt1, pt2);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instruction {
    Move { direction: Direction, distance: u32 },
    MoveForward(u32),
    RotateLeft,
    RotateRight,
    TurnAround,
}

pub fn pt1(instructions: &[Instruction]) -> i64 {
    let mut pos = i64x2::default();
    let mut dir = Direction::East;
    for mut instruction in instructions.iter().cloned() {
        if let Instruction::MoveForward(distance) = instruction {
            instruction = Instruction::Move {
                direction: dir,
                distance,
            };
        }
        match instruction {
            Instruction::Move {
                direction,
                distance,
            } => match direction {
                Direction::North => pos.y += distance as i64,
                Direction::East => pos.x += distance as i64,
                Direction::South => pos.y -= distance as i64,
                Direction::West => pos.x -= distance as i64,
            },
            Instruction::MoveForward(_) => unreachable!(),
            Instruction::RotateLeft => {
                dir = match dir {
                    Direction::North => Direction::West,
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
                };
            }
            Instruction::RotateRight => {
                dir = match dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                };
            }
            Instruction::TurnAround => {
                dir = match dir {
                    Direction::North => Direction::South,
                    Direction::East => Direction::West,
                    Direction::South => Direction::North,
                    Direction::West => Direction::East,
                };
            }
        }
    }

    pos.x.abs() + pos.y.abs()
}

pub fn pt2(instructions: &[Instruction]) -> i64 {
    let mut ship = i64x2::default();
    let mut waypoint = i64x2::new(10, 1);
    for &instruction in instructions {
        match instruction {
            Instruction::Move {
                direction,
                distance,
            } => match direction {
                Direction::North => waypoint.y += distance as i64,
                Direction::East => waypoint.x += distance as i64,
                Direction::South => waypoint.y -= distance as i64,
                Direction::West => waypoint.x -= distance as i64,
            },
            Instruction::MoveForward(multiplier) => {
                ship += waypoint * vec2!(multiplier as i64);
            }
            Instruction::RotateLeft => {
                waypoint = vec2!(-waypoint.y, waypoint.x);
            }
            Instruction::RotateRight => {
                waypoint = vec2!(waypoint.y, -waypoint.x);
            }
            Instruction::TurnAround => {
                waypoint = vec2!(-waypoint.x, -waypoint.y);
            }
        }
    }

    ship.x.abs() + ship.y.abs()
}

pub fn parse(input: &str) -> Result<Vec<Instruction>> {
    use framework::parser::*;
    let direction = map(one_of("NESW"), |c: char| match c {
        'N' => Direction::North,
        'E' => Direction::East,
        'S' => Direction::South,
        'W' => Direction::West,
        _ => unreachable!(),
    });
    let move_instruction = alt((
        map(pair(direction, take_u32), |(direction, distance)| {
            Instruction::Move {
                direction,
                distance,
            }
        }),
        map(preceded(char('F'), take_u32), |distance| {
            Instruction::MoveForward(distance)
        }),
    ));
    fn tag_value(
        tag_str: &'static str,
        value: Instruction,
    ) -> impl Fn(&str) -> IResult<Instruction> {
        move |input: &str| map(tag(tag_str), |_| value)(input)
    }
    let turn_instruction = alt((
        tag_value("L90", Instruction::RotateLeft),
        tag_value("L180", Instruction::TurnAround),
        tag_value("L270", Instruction::RotateRight),
        tag_value("R90", Instruction::RotateRight),
        tag_value("R180", Instruction::TurnAround),
        tag_value("R270", Instruction::RotateLeft),
    ));
    let instruction = alt((move_instruction, turn_instruction));
    separated_list1(char('\n'), instruction)(input).into_result()
}

#[cfg(test)]
const EXAMPLE: &str = "\
F10
N3
F7
R90
F11";

standard_tests!(
    parse []
    pt1 [ EXAMPLE => 25 ]
    pt2 [ EXAMPLE => 286 ]
);
