mod motion;

use std::collections::HashMap;
use std::fmt::Display;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use nom::IResult;
use nom::character::complete::{alphanumeric1, char, digit1};
use nom::combinator::all_consuming;
use nom::sequence::separated_pair;
use crate::day9::motion::{Move, Direction};


#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinate {
    x: isize, 
    y: isize
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.x, self.y)
    }
}

#[warn(clippy::nonminimal_bool)]
impl Coordinate {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn touching(&self, other: &Self) -> bool {
        (self.x == other.x + 1 || self.x == other.x || self.x == other.x - 1) && (self.y == other.y -1 || self.y == other.y || self.y == other.y + 1)
    }
}

// Line consists out of Direction and Steps
fn parse_line(line: &str) -> IResult<&str, Move> {
    separated_pair(alphanumeric1, char(' '), digit1)(line)
    .map(|(next_input, res)| (next_input, Move::new(res.0.into(), res.1.parse::<usize>().unwrap())))
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Option<Vec<Move>> {
    Some(
        input
        .lines()
        .map(|l| all_consuming(parse_line)(l).unwrap().1).collect()

    )
}

#[aoc(day9, part1)]
fn part1(input: &[Move]) -> Option<usize> {

    let mut head_position: Coordinate=Coordinate::new(0, 0);
    let mut tail_position: Coordinate=Coordinate::new(0, 0);
    let mut coordinates_used: HashMap<String, Coordinate> = HashMap::new();
    
    coordinates_used.insert(format!("{:03}_{:03}", tail_position.x, tail_position.y), tail_position);

    input
    .iter()
    .for_each(|m|
        {
            match m.direction {
                Direction::Up => { head_position.y -= m.steps as isize} ,
                Direction::Down =>{ head_position.y += m.steps as isize},
                Direction::Left => {head_position.x -= m.steps as isize},
                Direction::Right => {head_position.x += m.steps as isize},
            };

            for _ in 1..=m.steps {

                if !tail_position.touching(&head_position) {

                    if head_position.y < tail_position.y  {
                        tail_position.y -= 1;
                    }
                    if head_position.y > tail_position.y  {
                        tail_position.y += 1;
                    }

                    if head_position.x < tail_position.x  {
                        tail_position.x -= 1;
                    }
                    if head_position.x > tail_position.x  {
                        tail_position.x += 1;
                    }
                    coordinates_used.insert(format!("{:03}_{:03}", tail_position.x, tail_position.y), tail_position);
                    println!("{} - {}", head_position, tail_position);
                }
                else {
                    break;
                }
            }
        }
    );

    dbg!(&coordinates_used);

    Some(coordinates_used.keys().count())

}

#[aoc(day9, part2)]
fn part2(input: &[Move]) -> Option<usize> {

    let mut body_position: Vec<Coordinate>= vec![Coordinate::new(0, 0); 10];
    let mut coordinates_used: HashMap<String, &str> = HashMap::new();
    
    coordinates_used.insert(format!("{:03}_{:03}", 0, 0), "Does not matter");

    input
    .iter()
    .for_each(|m|
        {
            println!("== {} == ", m);
            println!();
            match m.direction {
                Direction::Up => { body_position[0].y -= m.steps as isize} ,
                Direction::Down =>{ body_position[0].y += m.steps as isize},
                Direction::Left => {body_position[0].x -= m.steps as isize},
                Direction::Right => {body_position[0].x += m.steps as isize},
            };

            for index in 1..body_position.len() {
                for _ in 1..=m.steps {

                    if !body_position[index].touching(&body_position[index-1]) {

                        if body_position[index-1].y < body_position[index].y  {
                            body_position[index].y -= 1;
                        }
                        if body_position[index-1].y > body_position[index].y  {
                            body_position[index].y += 1;
                        }

                        if body_position[index-1].x < body_position[index].x  {
                            body_position[index].x -= 1;
                        }
                        if body_position[index-1].x > body_position[index].x  {
                            body_position[index].x += 1;
                        }

                        coordinates_used.insert(format!("{:03}_{:03}", body_position.last().unwrap().x, body_position.last().unwrap().y), "Does not matter");
                        println!("M - {}: {} - {}", index, body_position[0], body_position[index]);
                    }
                    else {
                        println!("E - {}: {} - {}", index, body_position[0], body_position[index]);
                        break;
                    }
                }
            }
        }
    );

    dbg!(&coordinates_used);

    Some(coordinates_used.keys().count())
}

#[cfg(test)]
mod tests {

    use super::{input_generator, Direction, Move, part1, part2};

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
        Some( vec![
                Move { direction: Direction::Right, steps: 4 },
                Move { direction: Direction::Up, steps: 4 },
                Move { direction: Direction::Left, steps: 3 },
                Move { direction: Direction::Down, steps: 1 },
                Move { direction: Direction::Right, steps: 4 },
                Move { direction: Direction::Down, steps: 1 },
                Move { direction: Direction::Left, steps: 5 },
                Move { direction: Direction::Right, steps: 2 },
            ]))
    }

    #[test]
    fn test_part1() {

        assert_eq!(part1(
            &[
                Move { direction: Direction::Right, steps: 4 },
                Move { direction: Direction::Up, steps: 4 },
                Move { direction: Direction::Left, steps: 3 },
                Move { direction: Direction::Down, steps: 1 },
                Move { direction: Direction::Right, steps: 4 },
                Move { direction: Direction::Down, steps: 1 },
                Move { direction: Direction::Left, steps: 5 },
                Move { direction: Direction::Right, steps: 2 },
            ] ),
            Some(13)
        );
    }
    
    #[test]
    fn test_part2() {

        /*
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
 */
        assert_eq!(part2(
            &[
                Move { direction: Direction::Right, steps: 5 },
                Move { direction: Direction::Up, steps: 8 },
                Move { direction: Direction::Left, steps: 8 },
                Move { direction: Direction::Down, steps: 3 },
                Move { direction: Direction::Right, steps: 17 },
                Move { direction: Direction::Down, steps: 10 },
                Move { direction: Direction::Left, steps: 25 },
                Move { direction: Direction::Up, steps: 20 },
            ] ),
            Some(36)
        );

    }
}