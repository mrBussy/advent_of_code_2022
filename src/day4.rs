use std::ops::RangeInclusive;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

// Used the code found here: https://fasterthanli.me/series/advent-of-code-2022/part-4
trait  InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;

    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn overlaps_or_is_overlapped(&self, other: &Self) -> bool {
        self.overlaps(other) || other.overlaps(self)
    }
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<(RangeInclusive<i32>,RangeInclusive<i32>)> {
    input
    .lines()
    .map(|line|
        line.split(',')
        .map(|range| {
            range
                .split('-')
                .map(|number| number.parse::<i32>().expect("Range start/end should be a i32"))
                .collect_tuple::<(i32, i32)>()
                .map(|(start, end)| start..=end)
                .expect("each range should have a start end end") 
        })
        .collect_tuple::<(_, _)>()
        .expect("each line must have a pair of ranges")
    ).collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[(RangeInclusive<i32>,RangeInclusive<i32>)]) -> Option<i32> {
    
    Some(
    input
    .iter()
    .filter(|(range_a, range_b)| range_a.contains_or_is_contained(range_b))
    .count() as i32)
    
}
#[aoc(day4, part2)]
pub fn part2(input: &[(RangeInclusive<i32>,RangeInclusive<i32>)]) -> Option<i32> {
    
    Some(
    input
    .iter()
    .filter(|(range_a, range_b)| range_a.overlaps_or_is_overlapped(range_b))
    .count() as i32)
    
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use super::{input_generator, part1, part2};

    #[test]
    fn test_input() {
        assert_eq!(
            input_generator(
                "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"
            ),
            &[(RangeInclusive::new(2, 4), RangeInclusive::new(6,8)),
              (RangeInclusive::new(2,3),RangeInclusive::new(4,5)),
              (RangeInclusive::new(5,7),RangeInclusive::new(7,9)),
              (RangeInclusive::new(2,8),RangeInclusive::new(3,7)),
              (RangeInclusive::new(6, 6),RangeInclusive::new(4,6)),
              (RangeInclusive::new(2,6),RangeInclusive::new(4,8))
              ]
        )
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(
            &[(RangeInclusive::new(2, 4), RangeInclusive::new(6,8)),
              (RangeInclusive::new(2,3),RangeInclusive::new(4,5)),
              (RangeInclusive::new(5,7),RangeInclusive::new(7,9)),
              (RangeInclusive::new(2,8),RangeInclusive::new(3,7)),
              (RangeInclusive::new(6, 6),RangeInclusive::new(4,6)),
              (RangeInclusive::new(2,6),RangeInclusive::new(4,8))
              
              ]),
            Some(2))
    }
        #[test]
    pub fn test_part2() {
        assert_eq!(part2(
            &[(RangeInclusive::new(2, 4), RangeInclusive::new(6,8)),
              (RangeInclusive::new(2,3),RangeInclusive::new(4,5)),
              (RangeInclusive::new(5,7),RangeInclusive::new(7,9)),
              (RangeInclusive::new(2,8),RangeInclusive::new(3,7)),
              (RangeInclusive::new(6, 6),RangeInclusive::new(4,6)),
              (RangeInclusive::new(2,6),RangeInclusive::new(4,8))
              
              ]),
            Some(4))
    }
}
