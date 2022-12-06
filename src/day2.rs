use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

/*
    (1 for Rock, 2 for Paper, and 3 for Scissors)
 */
#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(usize, usize)> {
    input
    .lines()
    .map(|line| {
        line.split_ascii_whitespace().map(|token| 
            match token {
            "A" | "X" => 1,
            "B" | "Y" => 2,
            "C" | "Z" => 3,
            _ => 0
            })
            .collect_tuple::<(usize, usize)>()
            .unwrap()
    }
    ).collect()
}

/* 
Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z
This strategy guide predicts and recommends the following:

In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). 
This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).

In the second round, your opponent will choose Paper (B), and you should choose Rock (X). 
This ends in a loss for you with a score of 1 (1 + 0).

The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?
 */
#[aoc(day2, part1)]
pub fn part1(input: &[(usize, usize)]) -> usize {
    input
    .iter()
    .fold(0, |sum, line|
    {
        sum + line.1 + match (line.0, line.1) {  
            (1, 2) => {6}, (1, 3) => {0}, 
            (2, 3) => {6}, (2, 1) => {0}, 
            (3, 1) => {6}, (3, 2) => {0}, 
            (_, _) => {3} } 
    }
    )
}
/*
--- Part Two ---
The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: 
X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?

*/
#[aoc(day2, part2)]
pub fn part2(input: &[(usize, usize)]) -> usize {
    input
    .iter()
    .fold(0, |sum, line|
    {
        sum + match (line.0, line.1) {  
            (1, 1) => {3}, (1, 2) => {3 + 1}, (1, 3) => {6 + 2},
            (2, 1) => {1}, (2, 2) => {3 + 2}, (2, 3) => {6 + 3},
            (3, 1) => {2}, (3, 2) => {3 + 3}, (3, 3) => {6 + 1},
            (_, _) => {0} } 
    }
    )
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, input_generator};

    #[test]
    fn test_input() {
        assert_eq!(input_generator("A Y\nB X\nC Z"), vec![(1, 2), (2, 1), (3, 3)])
    }
    
    /*
    In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). 
This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).

In the second round, your opponent will choose Paper (B), and you should choose Rock (X). 
This ends in a loss for you with a score of 1 (1 + 0).

The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
*/
    #[test]
    fn test_part1() {
        assert_eq!( part1(
            &[(1, 2), (2, 1), (3, 3)]
        ), 15)
    }

    /*
    In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
    In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
    In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
    Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.
    */
    #[test]
    fn test_part2() {
        assert_eq!( part2(
            &[(1, 2), (2, 1), (3, 3)]
        ), 12)
    }
     
}