
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| 
          l.parse::<i32>().unwrap() ).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> usize {
    input.iter()
        .enumerate()
        .filter(|(index, value)| index > &0 && value > &&input[index - 1])
        .count()
}
#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> usize {
    let mut sliding_sum: Vec<i32> = Vec::new();
    for i in 0..input.len() - 2 {
        sliding_sum.push(input[i] + input[i + 1] + input[i + 2]);
    }

    part1(&sliding_sum)
        
}

#[cfg(test)]
mod tests {
    use super::{part1,part2};

    #[test]
    fn test_part1()  {
        assert_eq!(part1(&[199 ,
200, 
208, 
210, 
200, 
207, 
240, 
269, 
260, 
263]), 7);
    }

    #[test]
    fn test_part2()  {
        assert_eq!(part2(&[199 ,
200, 
208, 
210, 
200, 
207, 
240, 
269, 
260, 
263]), 5);
    }
}