use std::fmt::Display;

#[derive(Debug)]
pub struct Move {
    pub(crate) direction: Direction,
    pub(crate) steps: usize
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Moving {} {} steps", self.direction, self.steps)
    }
}

impl Move {
    pub(crate) fn new(direction: Direction, steps: usize) -> Self {
        Move { direction, steps }
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.direction == other.direction && self.steps == other.steps
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
