use crate::prelude::*;
use bitvec::prelude::*;
use std::ops::Range;

day!(16, parse => pt1, pt2);

#[derive(Debug, Clone)]
pub struct Field<'s> {
    name: &'s str,
    lower: Range<usize>,
    higher: Range<usize>,
}

pub type Ticket = Vec<u32>;

#[derive(Debug, Clone)]
pub struct Input<'s> {
    fields: Vec<Field<'s>>,
    own_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn get_valid_number(fields: &Vec<Field>) -> BitArray<LocalBits, [u64; 16]> {
    let mut valid_numbers = BitArray::<LocalBits, [u64; 16]>::default();
    for field in fields {
        valid_numbers[field.lower.clone()].set_all(true);
        valid_numbers[field.higher.clone()].set_all(true);
    }
    valid_numbers
}

pub fn pt1(input: &Input) -> u32 {
    let valid_numbers = get_valid_number(&input.fields);
    input
        .nearby_tickets
        .iter()
        .flatten()
        .cloned()
        .filter(|&nr| !valid_numbers[nr as usize])
        .sum()
}

pub fn pt2(input: &Input) -> Result<u64> {
    if input.fields.len() >= 32 {
        return Err(Error::InvalidInput("too many fields"));
    }
    let valid_numbers = get_valid_number(&input.fields);
    let valid_nearby_tickets = input
        .nearby_tickets
        .iter()
        .filter(|ticket| ticket.iter().all(|&nr| valid_numbers[nr as usize]))
        .collect::<Vec<_>>();

    // Create a mapping from field_index to potential column indices.
    let mut field_assignment = vec![Vec::new(); input.fields.len()];
    for (field_index, field) in input.fields.iter().enumerate() {
        for i in 0..input.fields.len() {
            if valid_nearby_tickets
                .iter()
                .map(|ticket| ticket[i] as usize)
                .all(|nr| field.lower.contains(&nr) || field.higher.contains(&nr))
            {
                field_assignment[field_index].push(i);
            }
        }
    }

    // A final assignment needs to be made on each field, and this is done by
    // iteratively selecting a field which only has one potential valid mapping,
    // assigning it, and then removing it from all the other fields.
    let mut pending_final_assignments = (0..input.fields.len()).collect::<Vec<usize>>();
    while !pending_final_assignments.is_empty() {
        if let Some((index_to_remove, field_index)) = pending_final_assignments
            .iter()
            .cloned()
            .enumerate()
            .find(|&(_, field_index)| field_assignment[field_index].len() == 1)
        {
            pending_final_assignments.swap_remove(index_to_remove);
            let assigned_number = field_assignment[field_index][0];
            for &other_field_index in &pending_final_assignments {
                let other_assignments = &mut field_assignment[other_field_index];
                if let Ok(index) = other_assignments.binary_search(&assigned_number) {
                    other_assignments.remove(index);
                }
            }
        } else {
            return Err(Error::NoSolution);
        }
    }

    // Now field_assignment[field_index] with a single element containing the
    // column index in each ticket.
    Ok(input
        .fields
        .iter()
        .enumerate()
        .filter(|(_, field)| field.name.starts_with("departure"))
        .map(|(field_index, _)| input.own_ticket[field_assignment[field_index][0]] as u64)
        .product())
}

pub fn parse(input: &str) -> Result<Input> {
    use framework::parser::*;
    fn range(input: &str) -> IResult<Range<usize>> {
        map(
            pair(terminated(take_u32, char('-')), take_u32),
            |(lower, upper)| lower as usize..(upper + 1) as usize,
        )(input)
    }
    fn ticket(input: &str) -> IResult<Vec<u32>> {
        separated_list1(char(','), take_u32)(input)
    }
    let field = map(
        tuple((
            terminated(take_while1(|c: char| c != ':'), tag(": ")),
            terminated(range, tag(" or ")),
            range,
        )),
        |(name, lower, higher)| Field {
            name,
            lower,
            higher,
        },
    );
    map(
        tuple((
            terminated(
                separated_list1(char('\n'), field),
                tag("\n\nyour ticket:\n"),
            ),
            terminated(ticket, tag("\n\nnearby tickets:\n")),
            separated_list1(char('\n'), ticket),
        )),
        |(fields, own_ticket, nearby_tickets)| Input {
            fields,
            own_ticket,
            nearby_tickets,
        },
    )(input)
    .into_result()
}

standard_tests!(
    parse []
    pt1 ["\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12" => 71]
    pt2 []
);
