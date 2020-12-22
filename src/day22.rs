use crate::prelude::*;
use std::{cmp::Ordering, collections::VecDeque};

day!(22, parse => pt1, pt2);

type Deck = VecDeque<u32>;

fn calculate_score(deck: &Deck) -> u32 {
    deck
    .iter()
    .rev()
    .enumerate()
    .map(|(i, &c)| (i as u32 + 1) * c)
    .sum()
}

pub fn pt1(input: &(Deck, Deck)) -> Result<u32> {
    let (mut a, mut b) = input.clone();
    while !a.is_empty() && !b.is_empty() {
        let ca = a.pop_front().unwrap();
        let cb = b.pop_front().unwrap();
        match ca.cmp(&cb) {
            Ordering::Less => {
                b.push_back(cb);
                b.push_back(ca);
            }
            Ordering::Equal => return Err(Error::InvalidInput("duplicate card")),
            Ordering::Greater => {
                a.push_back(ca);
                a.push_back(cb);
            }
        }
    }
    let winning_player = if !a.is_empty() { &a } else { &b };
    Ok(calculate_score(winning_player))
}

pub fn pt2(input: &(Deck, Deck)) -> Result<u32> {
    enum Winner {
        Player1(Deck),
        Player2(Deck),
    }

    fn play((mut a, mut b): (Deck, Deck)) -> Winner {
        let mut seen_setups = HashSet::<(Deck, Deck)>::new();
        loop {
            if a.is_empty() {
                return Winner::Player2(b);
            }
            if b.is_empty() || !seen_setups.insert((a.clone(), b.clone())) {
                return Winner::Player1(a);
            }
            let ca = a.pop_front().unwrap();
            let cb = b.pop_front().unwrap();
            let player_a_wins = if (ca as usize) <= a.len() && (cb as usize) <= b.len() {
                let new_a = a.iter().cloned().take(ca as usize).collect();
                let new_b = b.iter().cloned().take(cb as usize).collect();
                matches!(play((new_a, new_b)), Winner::Player1(_))
            } else {
                ca > cb
            };
            if player_a_wins {
                a.push_back(ca);
                a.push_back(cb);
            } else {
                b.push_back(cb);
                b.push_back(ca);
            }
        }
    }
    
    let deck = match play(input.clone()) {
        Winner::Player1(deck) => deck,
        Winner::Player2(deck) => deck,
    };
    Ok(calculate_score(&deck))
}

pub fn parse(input: &str) -> Result<(Deck, Deck)> {
    use framework::parser::*;
    fn deck(input: &str) -> IResult<Deck> {
        map(
            preceded(
                tuple((tag("Player "), one_of("12"), tag(":\n"))),
                separated_list1(char('\n'), take_u32),
            ),
            |v| v.into_iter().collect(),
        )(input)
    }
    let decks = pair(deck, preceded(tag("\n\n"), deck))(input).into_result()?;
    if decks.0.len() == decks.1.len() {
        return Ok(decks);
    } else {
        return Err(Error::InvalidInput("different deck sizes"));
    }
}

#[cfg(test)]
const EXAMPLE: &str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

standard_tests!(
    parse []
    pt1 [ EXAMPLE => 306 ]
    pt2 [ EXAMPLE => 291 ]
);
