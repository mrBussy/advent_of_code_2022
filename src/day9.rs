use std::collections::HashMap;
use std::fmt::Display;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use nom::IResult;
use nom::character::complete::{alphanumeric1, char, digit1};
use nom::combinator::all_consuming;
use nom::sequence::separated_pair;

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
        (self.x == other.x - 1 && self.y == other.y -1) ||
        (self.x == other.x - 1 && self.y == other.y) ||
        (self.x == other.x - 1 && self.y == other.y + 1) ||
        (self.x == other.x + 1 && self.y == other.y -1) ||
        (self.x == other.x + 1 && self.y == other.y) ||
        (self.x == other.x + 1 && self.y == other.y + 1) ||
        (self.x == other.x && self.y == other.y - 1) ||
        (self.x == other.x && self.y == other.y + 1) 
    }
}

#[derive(Debug)]
pub struct Field {
    position: Coordinate,
    head_visited: bool,
    tail_visited: bool
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.head_visited == other.head_visited && self.tail_visited == other.tail_visited
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'D' | 'd' => Self::Down,
            'U' | 'u' => Self::Up,
            'L' | 'l' => Self::Left,
            'R' | 'r' => Self::Right,
            _ =>  panic!("Unknown Direction: {}", c),
        }
    }
}
impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s.chars().next() {
            Some('D') | Some('d') => Self::Down,
            Some('U') | Some('u') => Self::Up,
            Some('L') | Some('l') => Self::Left,
            Some('R') | Some('r') => Self::Right,
            _ =>  panic!("Unknown Direction: {}", s),
        }
    }
}

#[derive(Debug)]
pub struct Move {
    direction: Direction,
    steps: usize
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Moving {} {} steps", self.direction, self.steps)
    }
}

impl Move {
    fn new(direction: Direction, steps: usize) -> Self {
        Move { direction, steps }
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.direction == other.direction && self.steps == other.steps
    }
}

#[derive(Debug)]
pub struct Play {
    head_position: Coordinate,
    tail_position: Coordinate,
    field: Vec<Field>,
    moves: Vec<Move>
}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        self.head_position == other.head_position && self.tail_position == other.tail_position && self.field == other.field && self.moves == other.moves
    }
}

// Line consists out of Direction and Steps
fn parse_line(line: &str) -> IResult<&str, Move> {
    separated_pair(alphanumeric1, char(' '), digit1)(line)
    .map(|(next_input, res)| (next_input, Move::new(res.0.into(), res.1.parse::<usize>().unwrap())))
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Option<Play> {
    Some(
        Play {
            head_position: Coordinate::new(1, 1),
            tail_position: Coordinate::new(1, 1),
            field: vec![],
            moves: 
                input
                .lines()
                .map(|l| all_consuming(parse_line)(l).unwrap().1).collect()
        }
    )
}

#[aoc(day9, part1)]
fn part1(input: &Play) -> Option<usize> {

    let mut head_position: Coordinate=Coordinate::new(0, 0);
    let mut tail_position: Coordinate=Coordinate::new(0, 0);
    let mut coordinates_used: HashMap<String, Coordinate> = HashMap::new();
    
    coordinates_used.insert(format!("{:03}_{:03}", tail_position.x, tail_position.y), tail_position);

    input
    .moves
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
#[cfg(test)]
mod tests {

    use super::{input_generator, Direction, Move, Play, part1, Coordinate};

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
        Some(Play { head_position: Coordinate::new(1, 1), tail_position: Coordinate::new(1, 1), field: vec![], 
            moves: vec![
                Move { direction: Direction::Right, steps: 4 },
                Move { direction: Direction::Up, steps: 4 },
                Move { direction: Direction::Left, steps: 3 },
                Move { direction: Direction::Down, steps: 1 },
                Move { direction: Direction::Right, steps: 4 },
                Move { direction: Direction::Down, steps: 1 },
                Move { direction: Direction::Left, steps: 5 },
                Move { direction: Direction::Right, steps: 2 },
            ] }))
    }

    #[test]
    fn test_part1() {

        assert_eq!(part1(
            &Play { head_position: Coordinate::new(1, 1), tail_position: Coordinate::new(1, 1), field: vec![], 
            moves: vec![
                Move { direction: Direction::Right, steps: 4 },
                Move { direction: Direction::Up, steps: 4 },
                Move { direction: Direction::Left, steps: 3 },
                Move { direction: Direction::Down, steps: 1 },
                Move { direction: Direction::Right, steps: 4 },
                Move { direction: Direction::Down, steps: 1 },
                Move { direction: Direction::Left, steps: 5 },
                Move { direction: Direction::Right, steps: 2 },
            ] }),
            Some(13)
        );

    }
}