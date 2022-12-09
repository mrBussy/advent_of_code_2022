use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Option<usize> {

    None
}

#[cfg(test)]
mod tests {

    use super::{input_generator};

    #[test]
    fn test_input() { 
        assert_eq!(input_generator(
"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
        ),
        Some(13))
    }
}