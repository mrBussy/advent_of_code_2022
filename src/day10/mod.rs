use aoc_runner_derive::{aoc_generator, aoc};
use nom::{bytes::complete::{tag_no_case, tag}, combinator::{map, value}, branch::alt, sequence::preceded, IResult};

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
    register_x: i64
}

impl MachineState {
    pub(crate) fn new() -> Self {
        MachineState { cycle: 0, register_x: 1 }
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

#[cfg(test)]
mod tests {

    use super::{input_generator, part1, Instruction};

    #[test]
    fn test_input() { 
        assert_eq!(input_generator(
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"),
            Some(vec![Instruction::Addx(15), 
            Instruction::Addx(-11), 
            Instruction::Addx(6), 
            Instruction::Addx(-3), 
            Instruction::Addx(5), 
            Instruction::Addx(-1), 
            Instruction::Addx(-8), 
            Instruction::Addx(13),
            Instruction::Addx(4), 
            Instruction::Noop, 
            Instruction::Addx(-1), 
            Instruction::Addx(5), 
            Instruction::Addx(-1), 
            Instruction::Addx(5), 
            Instruction::Addx(-1), 
            Instruction::Addx(5), 
            Instruction::Addx(-1), 
            Instruction::Addx(5), 
            Instruction::Addx(-1), 
            Instruction::Addx(-35), 
            Instruction::Addx(1), 
            Instruction::Addx(24), 
            Instruction::Addx(-19), 
            Instruction::Addx(1), 
            Instruction::Addx(16), 
            Instruction::Addx(-11), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(21), 
            Instruction::Addx(-15), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(-3), 
            Instruction::Addx(9), 
            Instruction::Addx(1), 
            Instruction::Addx(-3), 
            Instruction::Addx(8), 
            Instruction::Addx(1), 
            Instruction::Addx(5), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(-36), 
            Instruction::Noop, 
            Instruction::Addx(1), 
            Instruction::Addx(7), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(2), 
            Instruction::Addx(6), 
            Instruction::Noop, 
            Instruction::Noop,
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(1), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(7), 
            Instruction::Addx(1), 
            Instruction::Noop, 
            Instruction::Addx(-13), 
            Instruction::Addx(13), 
            Instruction::Addx(7), 
            Instruction::Noop, 
            Instruction::Addx(1), 
            Instruction::Addx(-33), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(2), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(8), 
            Instruction::Noop, 
            Instruction::Addx(-1), 
            Instruction::Addx(2), 
            Instruction::Addx(1), 
            Instruction::Noop, 
            Instruction::Addx(17), 
            Instruction::Addx(-9), 
            Instruction::Addx(1), 
            Instruction::Addx(1),
            Instruction::Addx(-3), 
            Instruction::Addx(11), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(1), 
            Instruction::Noop, 
            Instruction::Addx(1), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(-13), 
            Instruction::Addx(-19), 
            Instruction::Addx(1), 
            Instruction::Addx(3), 
            Instruction::Addx(26), 
            Instruction::Addx(-30),
            Instruction::Addx(12), 
            Instruction::Addx(-1), 
            Instruction::Addx(3), 
            Instruction::Addx(1),
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(-9), 
            Instruction::Addx(18), 
            Instruction::Addx(1), 
            Instruction::Addx(2), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(9), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(-1), 
            Instruction::Addx(2), 
            Instruction::Addx(-37), 
            Instruction::Addx(1), 
            Instruction::Addx(3), 
            Instruction::Noop, 
            Instruction::Addx(15), 
            Instruction::Addx(-21), 
            Instruction::Addx(22), 
            Instruction::Addx(-6), 
            Instruction::Addx(1), 
            Instruction::Noop, 
            Instruction::Addx(2), 
            Instruction::Addx(1), 
            Instruction::Noop, 
            Instruction::Addx(-10), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(20), 
            Instruction::Addx(1), 
            Instruction::Addx(2), 
            Instruction::Addx(2), 
            Instruction::Addx(-6), 
            Instruction::Addx(-11), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop])
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[Instruction::Addx(15), 
            Instruction::Addx(-11), 
            Instruction::Addx(6), 
            Instruction::Addx(-3), 
            Instruction::Addx(5), 
            Instruction::Addx(-1), 
            Instruction::Addx(-8), 
            Instruction::Addx(13),
            Instruction::Addx(4), 
            Instruction::Noop, 
            Instruction::Addx(-1), 
            Instruction::Addx(5), 
            Instruction::Addx(-1), 
            Instruction::Addx(5), 
            Instruction::Addx(-1), 
            Instruction::Addx(5), 
            Instruction::Addx(-1), 
            Instruction::Addx(5), 
            Instruction::Addx(-1), 
            Instruction::Addx(-35), 
            Instruction::Addx(1), 
            Instruction::Addx(24), 
            Instruction::Addx(-19), 
            Instruction::Addx(1), 
            Instruction::Addx(16), 
            Instruction::Addx(-11), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(21), 
            Instruction::Addx(-15), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(-3), 
            Instruction::Addx(9), 
            Instruction::Addx(1), 
            Instruction::Addx(-3), 
            Instruction::Addx(8), 
            Instruction::Addx(1), 
            Instruction::Addx(5), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(-36), 
            Instruction::Noop, 
            Instruction::Addx(1), 
            Instruction::Addx(7), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(2), 
            Instruction::Addx(6), 
            Instruction::Noop, 
            Instruction::Noop,
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(1), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(7), 
            Instruction::Addx(1), 
            Instruction::Noop, 
            Instruction::Addx(-13), 
            Instruction::Addx(13), 
            Instruction::Addx(7), 
            Instruction::Noop, 
            Instruction::Addx(1), 
            Instruction::Addx(-33), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(2), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(8), 
            Instruction::Noop, 
            Instruction::Addx(-1), 
            Instruction::Addx(2), 
            Instruction::Addx(1), 
            Instruction::Noop, 
            Instruction::Addx(17), 
            Instruction::Addx(-9), 
            Instruction::Addx(1), 
            Instruction::Addx(1),
            Instruction::Addx(-3), 
            Instruction::Addx(11), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(1), 
            Instruction::Noop, 
            Instruction::Addx(1), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(-13), 
            Instruction::Addx(-19), 
            Instruction::Addx(1), 
            Instruction::Addx(3), 
            Instruction::Addx(26), 
            Instruction::Addx(-30),
            Instruction::Addx(12), 
            Instruction::Addx(-1), 
            Instruction::Addx(3), 
            Instruction::Addx(1),
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(-9), 
            Instruction::Addx(18), 
            Instruction::Addx(1), 
            Instruction::Addx(2), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(9), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(-1), 
            Instruction::Addx(2), 
            Instruction::Addx(-37), 
            Instruction::Addx(1), 
            Instruction::Addx(3), 
            Instruction::Noop, 
            Instruction::Addx(15), 
            Instruction::Addx(-21), 
            Instruction::Addx(22), 
            Instruction::Addx(-6), 
            Instruction::Addx(1), 
            Instruction::Noop, 
            Instruction::Addx(2), 
            Instruction::Addx(1), 
            Instruction::Noop, 
            Instruction::Addx(-10), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Addx(20), 
            Instruction::Addx(1), 
            Instruction::Addx(2), 
            Instruction::Addx(2), 
            Instruction::Addx(-6), 
            Instruction::Addx(-11), 
            Instruction::Noop, 
            Instruction::Noop, 
            Instruction::Noop]
        ),
        Some(13140))
    }
}