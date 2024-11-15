use std::collections::{HashMap, HashSet};

use itertools::Itertools;

// Template for new days
use crate::{
    solution::Solution,
    utils::bfs::{bfs, Graph},
};

pub struct Day9;

/// let's iteratively create a lookup table just to make sure we catch the correct solution
/// returns beetle count
struct BeetleCountCalculator {
    memo: HashMap<i64, i64>,
    current_iterations: i64,
    fully_filled_until: i64,
}

impl BeetleCountCalculator {
    pub fn new() -> Self {
        Self {
            memo: HashMap::from([(0, 0)]),
            current_iterations: 0,
            fully_filled_until: 0,
        }
    }

    pub fn get(&mut self, brightness: i64, stamps: &[i64]) -> i64 {
        while !self.memo.contains_key(&brightness) {
            let result = minimum_stamps_bfs(brightness, stamps);
            self.memo.insert(brightness, result);
            return result;
        }
        *self.memo.get(&brightness).unwrap()
    }

    /// yeah yeah, this is an explosion of combinations in later iterations and we do this for all keys
    /// sue me
    fn new_iteration(&mut self, stamps: &[i64]) {
        let mut new_beetle_counts = Vec::new();
        for key in self
            .memo
            .keys()
            .filter(|key| **key > self.fully_filled_until)
        {
            new_beetle_counts.extend(
                stamps
                    .iter()
                    .map(|stamp| stamp + key)
                    .filter(|new_value| !self.memo.contains_key(new_value)),
            );
        }
        // all of the beetle counts should be the shortest possible path because we built them up from smallest paths
        self.memo.extend(
            new_beetle_counts
                .into_iter()
                .map(|num| (num, self.current_iterations + 1)),
        );
        // update fully_filled_until
        while self.memo.contains_key(&(self.fully_filled_until + 1)) {
            self.fully_filled_until += 1;
        }
        self.current_iterations += 1;
    }

    pub fn sparkball_combinations(&mut self, brightness: i64, stamps: &[i64]) -> i64 {
        let half_brightness = brightness / 2;
        // iterator based approach with nested closure did not work, so let's use for loops for now
        // at least this inefficient solutions should work okay once the values are precomputed
        let mut min = i64::MAX;
        for a in (half_brightness - 50)..=(half_brightness) {
            let b = brightness - a;
            if (b - a).abs() > 100 {
                continue;
            }
            let sum = self.get(a, stamps) + self.get(b, stamps);
            if sum < min {
                println!("Better combination: {a} and {b}: {sum}");
                min = sum;
            }
        }
        min
    }
}

/// this greedy algo works for part 1 but not 2
fn minimum_stamps(goal: i64, stamps: &[i64]) -> Vec<i64> {
    let mut times_stamped = vec![0; stamps.len()];
    let mut remaining = goal;
    for (i, stamp) in stamps.iter().enumerate() {
        while remaining >= *stamp {
            remaining -= stamp;
            times_stamped[i] += 1;
        }
    }
    times_stamped
}

fn minimum_stamps_bfs(goal: i64, stamps: &[i64]) -> i64 {
    let mut minimizer = BfsBeetlesMinimizer {
        stamps: stamps.iter().cloned().collect(),
    };
    let path = bfs(&mut minimizer, goal, 0).unwrap().collect_vec();
    path.iter().tuple_windows().map(|(a, b)| a - b).len() as i64
}

#[derive(Debug)]
struct BfsBeetlesMinimizer {
    stamps: Vec<i64>,
}

impl Graph for BfsBeetlesMinimizer {
    /// just the remaining brightness
    type Node = i64;

    fn neighbors(&mut self, n: &Self::Node) -> impl Iterator<Item = Self::Node> {
        let n = n.clone();
        self.stamps
            .iter()
            .filter_map(move |stamp| if *stamp <= n { Some(n - stamp) } else { None })
    }
}

impl Solution<i64> for Day9 {
    const DAY: usize = 9;

    fn part1(&self, input: &str) -> Option<i64> {
        Some(
            input
                .lines()
                .map(|s| minimum_stamps_bfs(s.parse().unwrap(), &[10, 5, 3, 1]))
                .sum(),
        )
    }

    fn part2(&self, input: &str) -> Option<i64> {
        let mut calc = BeetleCountCalculator::new();
        Some(
            input
                .lines()
                .map(|s| {
                    minimum_stamps_bfs(s.parse().unwrap(), &[1, 3, 5, 10, 15, 16, 20, 24, 25, 30])
                })
                .sum(),
        )
    }

    fn part3(&self, input: &str) -> Option<i64> {
        let mut calc = BeetleCountCalculator::new();
        Some(
            input
                .lines()
                .map(|s| {
                    calc.sparkball_combinations(
                        s.parse().unwrap(),
                        &[
                            1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
                        ],
                    )
                })
                .inspect(|res| println!("{:?}", res))
                .sum(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_solution() -> impl Solution<i64> {
        Day9
    }

    #[test]
    fn test_part1() {
        let test_input = r#"2
4
7
16"#;
        let solution = get_solution();
        assert_eq!(solution.part1(test_input), Some(10));
    }

    #[test]
    fn test_part2() {
        let solution = get_solution();
        let test_input = r#"33
41
55
99"#;
        assert_eq!(solution.part2(test_input), Some(10));
    }

    #[test]
    fn test_part3() {
        let solution = get_solution();
        let test_input = r#"156488
352486
546212"#;
        assert_eq!(solution.part3(test_input), Some(10449));
    }
}
