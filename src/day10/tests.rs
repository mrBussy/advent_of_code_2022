
#[cfg(test)]

use super::{input_generator, part1, part2, Instruction};

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


#[test]
fn test_part2() {
    assert_eq!(part2(&[Instruction::Addx(15), 
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
    Some(240))
}
