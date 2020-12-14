use std::collections::HashMap;

use crate::prelude::*;
day!(14, parse => pt1, pt2);

const BIT_SIZE_MASK: u64 = (1u64 << 36) - 1;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Opcode {
    Mask { mask: u64, value: u64 },
    Assign { address: u64, value: u64 },
}

pub fn pt1(opcodes: &[Opcode]) -> u64 {
    let mut current_mask = 0;
    let mut current_mask_value = 0;
    let mut memory = HashMap::new();
    for &opcode in opcodes {
        match opcode {
            Opcode::Mask {mask, value} => {
                current_mask = mask;
                current_mask_value = value;
            }
            Opcode::Assign {address, value} => {
                let value = (value & !current_mask) | current_mask_value;
                memory.insert(address, value);
            }
        }
    }
    memory.values().sum()
}

pub fn pt2(opcodes: &[Opcode]) -> u64 {
    let mut or_mask = 0;
    let mut and_mask = 0;
    let mut floating_or_masks = Vec::new();
    let mut floating_or_masks_backbuffer = Vec::new();
    
    let mut memory = HashMap::new();
    for &opcode in opcodes {
        match opcode {
            Opcode::Mask {mask, value} => {
                floating_or_masks.clear();
                floating_or_masks.push(0);
                let mut remaining_mask = !mask & BIT_SIZE_MASK;
                let mut current_bit = 1;
                while remaining_mask != 0 {
                    if remaining_mask & 1 == 1 {
                        floating_or_masks_backbuffer.clear();
                        for &floating_or_mask in &floating_or_masks {
                            floating_or_masks_backbuffer.push(floating_or_mask);
                            floating_or_masks_backbuffer.push(floating_or_mask | current_bit);
                        }
                        std::mem::swap(&mut floating_or_masks, &mut floating_or_masks_backbuffer);
                    }
                    remaining_mask >>= 1;
                    current_bit <<= 1;
                }
                or_mask = value;
                and_mask = mask;
            }
            Opcode::Assign {address, value} => {
                let address = (address & and_mask) | or_mask;
                for &floating_or_mask in &floating_or_masks {
                    memory.insert(address | floating_or_mask, value);
                }
            }
        }
    }
    memory.values().sum()
}

pub fn parse(input: &str) -> Result<Vec<Opcode>> {
    use framework::parser::*;
    let mask = map(
        preceded(
            tag("mask = "),
            fold_many1(one_of("X01"), (0, 0), |(mask, value), current| {
                (
                    (mask << 1) | if current != 'X' { 1 } else { 0 },
                    (value << 1) | if current == '1' { 1 } else { 0 },
                )
            }),
        ),
        |(mask, value)| Opcode::Mask { mask, value },
    );
    let assign = map(
        pair(
            preceded(tag("mem["), take_u64),
            preceded(tag("] = "), take_u64),
        ),
        |(address, value)| Opcode::Assign { address, value },
    );
    separated_list1(char('\n'), alt((mask, assign)))(input).into_result()
}

standard_tests!(
    parse []
    pt1 [
        "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0" => 165
]
    pt2 [
        "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1" => 208
    ]
);
