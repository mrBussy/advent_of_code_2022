use std::vec;

use aoc_runner_derive::{aoc_generator, aoc};
use nom::{bytes::complete::{tag_no_case, tag}, combinator::{map, value}, branch::alt, sequence::preceded, IResult};
mod tests;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Noop,
    Addx(i32)
}

impl Instruction {
    pub(crate) fn parse(instruction: &str) -> IResult<&str, Self> {
        let noop_tag = tag_no_case("noop");
        let addx_parser = preceded(tag("addx "), nom::character::complete::i32);

        alt((
            value(Instruction::Noop, noop_tag), 
            map(addx_parser, Instruction::Addx)))(instruction)
    }

    pub(crate) fn get_cycles(&self) -> i32 {
        match self {
            Instruction::Noop => {1}
            _ =>  {2}
        }
    }

    pub(crate) fn execute(&self, machine_state: MachineState, instruction_cycle: i32) -> Result<i64, &'static str> {
        let mut register_x = machine_state.register_x;

        match self.to_owned() {
            Self::Noop => {}
            Self::Addx(value) => { if instruction_cycle == 2 {register_x += value as i64} }
        }
        Ok(register_x)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MachineState {
    cycle: u64,
    register_x: i64,
}

impl MachineState {
    pub(crate) fn new() -> Self {
        MachineState { cycle: 0, register_x: 1 }
    }
}

const DISPLAY_MASK: u64 = 0b1111111111111111111111111111111111111111;
struct Crt {
    display_lines: Vec<u64>,
}

impl Crt {

    fn cycle_mask(cycle: u32) -> u64 {
        (0b1000000000000000000000000000000000000000 >> (cycle % 40)) & DISPLAY_MASK
    }

    fn sprite_value(pos: i32) -> u64 {
        let model = 0b11100000000000000000000000000000000000000_u64;
        let shifted;
        if pos < 0 {
            (shifted, _) = model.overflowing_shl((-pos).try_into().unwrap());
        } else {
            (shifted, _) = model.overflowing_shr(pos.try_into().unwrap());
        }
        shifted & DISPLAY_MASK
    }

    fn draw(&mut self, cycle: u32, register_x: u32) {
        let crt_line = (cycle / 40) as usize;
        if crt_line + 1 > self.display_lines.len() {
            self.display_lines.push(0);
        }
        let crt_line = self.display_lines.get_mut(crt_line).unwrap();
        let cycle_mask = Self::cycle_mask(cycle);
        let sprite = Self::sprite_value(register_x as _);
        *crt_line |= cycle_mask & sprite;
    }

    fn display(&self) {
        for line in &self.display_lines {
            println!("{}", format!("{:040b}", line)
            .replace('1', "🎅")
            .replace('0', "🎄"));
        }
    }
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Option<Vec<Instruction>> {
    Some(
        input
        .lines()
        .map(|line|
            Instruction::parse(line).unwrap().1
        )
        .collect()
    )
}

#[aoc[day10, part1]]
pub fn part1(instructions: &[Instruction]) -> Option<i64> {

    let mut machine_state = MachineState::new();
    let mut signal_strengths: Vec<i64> = vec![];
    let take_shot = &[20, 60, 100, 140, 180, 220, 260, 300];

    for instruction in instructions {
        for cycle in 1..=instruction.get_cycles() {
            machine_state.cycle += 1;
            if take_shot.contains(&machine_state.cycle)  {
                let value = machine_state.cycle as i64 * machine_state.register_x;
                signal_strengths.push(value);
                println!("During the {0} cycle, register X has the value {1}, so the signal strength is {0} * {1} = {2}.", machine_state.cycle, machine_state.register_x, value);
            }
            machine_state.register_x = instruction.execute(machine_state, cycle).unwrap();
        }
    }
    Some(signal_strengths.iter().sum())
}

/*
    The sprite starts drawing at the machine.regster_x pos
*/
#[aoc[day10, part2]]
pub fn part2(instructions: &[Instruction]) -> Option<i64> {

    let mut machine_state = MachineState::new();
    let mut crt: Crt = Crt { display_lines: Vec::new() };

    for instruction in instructions {
        for cycle in 1..=instruction.get_cycles() {
            crt.draw(machine_state.cycle as u32, machine_state.register_x as u32);
            machine_state.cycle += 1;
            machine_state.register_x = instruction.execute(machine_state, cycle).unwrap();
        }
    }
    crt.display();
    Some(machine_state.cycle as i64)
}
