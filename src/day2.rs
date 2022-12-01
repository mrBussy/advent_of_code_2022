use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<i32> {
    Vec::new()
}

#[aoc(day2, part1)]
pub fn part1(input: &[i32]) -> i32 {
    0
}

#[aoc(day2, part2)]
pub fn part2(input: &[i32]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::{part1,part2, input_generator};

    #[test]
    fn test_input() {
        assert_eq!(input_generator(""), vec![1])
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[1]), 0)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&[1]), 0)
    }
    
}