use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use regex::Regex;

#[derive(Debug)]
pub struct Ship {

    stacks: Vec<Vec<char>>,
    moves: Vec<(i8, i8, i8)>
}

impl PartialEq for Ship {
    fn eq(&self, other: &Self) -> bool {
        self.stacks == other.stacks && self.moves == other.moves
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Result<Ship, &'static str> {

    let mut build_moves: Vec<(i8, i8, i8)> = vec![];
    let mut stacks: Vec<Vec<char>> = vec![vec![]];
    let move_regex = Regex::new(r"^move\s(\d+)\sfrom\s(\d+)\sto\s(\d+).*").unwrap();

    input
    .lines()
        .for_each(|line|
        {
            // Moves
            if move_regex.captures(line).is_some() {

                build_moves.push(
                    (
                        move_regex.captures(line).unwrap().get(1).map(|value| value.as_str().parse::<i8>().expect("value")).unwrap(),
                        move_regex.captures(line).unwrap().get(2).map(|value| value.as_str().parse::<i8>().expect("value")).unwrap(),
                        move_regex.captures(line).unwrap().get(3).map(|value| value.as_str().parse::<i8>().expect("value")).unwrap()
                    ))
            } 
        }
    );

    input
    .lines()
    .rev()
    .for_each(|line| {
            if line.contains('[') {
                line
                .chars()
                .enumerate()
                .for_each(|(index, value)|
                    if value.is_alphabetic() { 
                        stacks[(index/4)].push(value);
                    }
                );
            }
            // numbers only
            if !line.contains('[') && !line.contains('m') && !line.is_empty() {
                stacks = Vec::new();
                for char in line.chars() {
                    if char.is_alphanumeric() {
                        stacks.push(Vec::new())
                    }
                }
            }
        }
    );

    Ok(Ship { stacks: stacks, moves: build_moves })
}

#[aoc(day5, part1)]
pub fn part1(input: &Ship) -> Option<String> {
    
    let mut stacks=input.stacks.clone();

    input
    .moves
    .iter()
    .for_each(|(number_of_crates, from, to)|
        for _ in 1..=*number_of_crates {
            let char = stacks[(*from-1) as usize].pop().unwrap();
            stacks[(*to-1) as usize].push(char)
        }
    );

    Some(

        stacks
        .iter()
        .fold(String::new(), |mut val, stack| {
            val.push(*stack.last().unwrap());
            val
            }
        ) 
    )
}


#[aoc(day5, part2)]
pub fn part2(input: &Ship) -> Option<String> {
    
    let mut stacks=input.stacks.clone();

    input
    .moves
    .iter()
    .for_each(|(number_of_crates, from, to)|
    {
        let mut buffer: Vec<char> = Vec::new();
        for _ in 1..=*number_of_crates {
            buffer.push(stacks[(*from-1) as usize].pop().unwrap());
        }
        for _ in 1..=*number_of_crates {
            stacks[(*to-1) as usize].push(buffer.pop().unwrap());
        }
    }
    );

    Some(

        stacks
        .iter()
        .fold(String::new(), |mut val, stack| {
            val.push(*stack.last().unwrap());
            val
            }
        ) 
    )
}

#[cfg(test)]
mod tests {

    use super::{input_generator, part1, part2, Ship};

    #[test]
    fn test_input() {
        assert_eq!(
            input_generator(
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"
            ).unwrap(),
            Ship{ moves: vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)],
                 stacks: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
            }
        )
    }

    
    #[test]
    pub fn test_part1() {
        assert_eq!(part1(
            &Ship{ moves: vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)],
                 stacks: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
            }),
            Some(String::from("CMZ")))
    }
    
    #[test]
    pub fn test_part2() {
        assert_eq!(part2(
            &Ship{ moves: vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)],
                 stacks: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
            }),
            Some(String::from("MCD")))
    }
}
