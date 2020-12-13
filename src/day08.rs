use num::ToPrimitive;

use crate::prelude::*;
day!(8, parse => pt1, pt2);

pub type Int = i64;
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct State {
    pub ip: i64,
    pub acc: i64,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    acc(Int),
    jmp(Int),
    nop(Int),
}

impl State {
    // Returns whether an instruction was executed
    fn advance(&mut self, instructions: &[Instruction]) -> bool {
        if let Some(&instruction) = self.ip.to_usize().and_then(|idx| instructions.get(idx)) {
            match instruction {
                Instruction::acc(n) => {
                    self.acc += n;
                    self.ip += 1;
                }
                Instruction::jmp(n) => {
                    self.ip += n;
                }
                Instruction::nop(_) => {
                    self.ip += 1;
                }
            }
            true
        } else {
            false
        }
    }
}

pub fn get_acc_when_looping(instructions: &[Instruction]) -> std::result::Result<Int, State> {
    let mut has_executed_instruction = vec![false; instructions.len()];
    let mut state = State::default();
    loop {
        if let Some(has_executed_this_instruction) = state
            .ip
            .to_usize()
            .and_then(|index| has_executed_instruction.get_mut(index))
        {
            if *has_executed_this_instruction {
                return Ok(state.acc);
            }
            *has_executed_this_instruction = true;
        }
        if !state.advance(instructions) {
            return Err(state);
        }
    }
}

pub fn pt1(instructions: &[Instruction]) -> Result<Int> {
    get_acc_when_looping(instructions)
        .ok()
        .ok_or(Error::NoSolution)
}

pub fn pt2(instructions: &[Instruction]) -> Result<Int> {
    let mut instructions = instructions.to_vec();
    let expected_ip = instructions.len() as Int;
    for i in 0..instructions.len() {
        let old = instructions[i];
        let new = match old {
            Instruction::acc(_) => continue,
            Instruction::jmp(n) => Instruction::nop(n),
            Instruction::nop(n) => Instruction::jmp(n),
        };

        instructions[i] = new;
        if let Err(state) = get_acc_when_looping(&instructions) {
            if state.ip == expected_ip {
                return Ok(state.acc);
            }
        }
        instructions[i] = old;
    }
    Err(Error::NoSolution)
}

pub fn parse(input: &str) -> Result<Vec<Instruction>> {
    use framework::parser::*;
    let instr_tag = alt((tag("acc"), tag("jmp"), tag("nop")));
    let instr = map(
        pair(terminated(instr_tag, char(' ')), take_i64),
        |(tag, nr)| match tag {
            "acc" => Instruction::acc(nr),
            "jmp" => Instruction::jmp(nr),
            "nop" => Instruction::nop(nr),
            _ => unreachable!(),
        },
    );
    separated_list1(char('\n'), instr)(input).into_result()
}

#[cfg(test)]
const EXAMPLE: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

standard_tests!(
    parse [ EXAMPLE => vec![
        Instruction::nop(0),
        Instruction::acc(1),
        Instruction::jmp(4),
        Instruction::acc(3),
        Instruction::jmp(-3),
        Instruction::acc(-99),
        Instruction::acc(1),
        Instruction::jmp(-4),
        Instruction::acc(6),
    ]]
    pt1 [ EXAMPLE => 5 ]
    pt2 [ EXAMPLE => 8 ]
);
