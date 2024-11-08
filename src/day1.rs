use core::panic;

use itertools::Itertools;

use crate::solution::Solution;

pub struct Day1;

fn p2_map(c: char) -> Option<i32> {
    match c {
        'A' => Some(0),
        'B' => Some(1),
        'C' => Some(3),
        'D' => Some(5),
        'x' => None,
        _ => panic!("Invalid input"),
    }
}

impl Solution<i32> for Day1 {
    const DAY: usize = 1;
    fn part1(&self, input: &str) -> Option<i32> {
        Some(
            input
                .chars()
                .map(|c| match c {
                    'A' => 0,
                    'B' => 1,
                    'C' => 3,
                    _ => panic!("Invalid input"),
                })
                .sum(),
        )
    }

    fn part2(&self, input: &str) -> Option<i32> {
        Some(
            input
                .chars()
                .tuples()
                .map(|(a, b)| {
                    let a = p2_map(a);
                    let b = p2_map(b);
                    match (a, b) {
                        (Some(a), Some(b)) => a + b + 2,
                        (Some(a), None) => a,
                        (None, Some(b)) => b,
                        (None, None) => 0,
                    }
                })
                .sum(),
        )
    }

    fn part3(&self, input: &str) -> Option<i32> {
        Some(
            input
                .chars()
                .tuples()
                .map(|(a, b, c)| {
                    let a = p2_map(a);
                    let b = p2_map(b);
                    let c = p2_map(c);
                    // one creature -> no bonus
                    // two creatures -> 1 bonus for each
                    // three creatures -> 2 bonus for all
                    match (a, b, c) {
                        (Some(a), Some(b), Some(c)) => a + b + c + 6,
                        (Some(a), Some(b), None) => a + b + 2,
                        (Some(a), None, Some(c)) => a + c + 2,
                        (None, Some(b), Some(c)) => b + c + 2,
                        (Some(a), None, None) => a,
                        (None, Some(b), None) => b,
                        (None, None, Some(c)) => c,
                        (None, None, None) => 0,
                    }
                })
                .sum(),
        )
    }
}
