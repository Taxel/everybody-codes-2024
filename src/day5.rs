use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    str::FromStr,
};

use crate::solution::Solution;

pub struct Day5;

struct Day5World {
    // we need to insert at arbitrary positions and pop from the front
    // LinkedList sounds like a good choice, yet the rust API in the stdlib does not -> VecDeque
    columns: [VecDeque<usize>; 4],
    current_dancing_column: usize,
}

impl Day5World {
    pub fn new() -> Self {
        Self {
            columns: [
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
            ],
            current_dancing_column: 0,
        }
    }

    /// One step, returns the shouted number at the end
    fn dance(&mut self) -> String {
        // remove first clapper from current column
        let clapper = self.columns[self.current_dancing_column]
            .pop_front()
            .unwrap();
        // increment current_dancing_column immediately, this is where the clapper will be inserted
        self.current_dancing_column = (self.current_dancing_column + 1) % 4;

        let current_column_len = self.columns[self.current_dancing_column].len();

        // subtract full rounds (assumption clapper circles around is not explicitly stated imho)
        let clapper_idx = clapper % (current_column_len * 2);
        if clapper_idx == 0 {
            // insert at 1
            self.columns[self.current_dancing_column].insert(1, clapper);
        } else if clapper_idx <= current_column_len {
            // insert at index clapper-1
            self.columns[self.current_dancing_column].insert(clapper_idx - 1, clapper);
        } else {
            // insert at index clapper - current_column_len
            self.columns[self.current_dancing_column].insert(
                current_column_len - (clapper_idx - current_column_len - 1),
                clapper,
            );
        }

        // shouted number
        let shouted: String = self.columns.iter().map(|c| c[0].to_string()).collect();
        shouted
    }
}

impl FromStr for Day5World {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut columns = [
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
        ];
        for line in s.lines() {
            let mut clappers = line.split_whitespace().map(|s| s.parse().unwrap());
            for column in &mut columns {
                column.push_back(clappers.next().unwrap());
            }
        }

        Ok(Self {
            columns,
            current_dancing_column: 0,
        })
    }
}

impl Display for Day5World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;
        loop {
            let mut outputted = false;
            for column in &self.columns {
                if let Some(clapper) = column.get(i) {
                    write!(f, "{} ", clapper)?;
                    outputted = true;
                } else {
                    write!(f, "  ")?;
                }
            }
            i += 1;
            writeln!(f)?;
            if !outputted {
                break;
            }
        }
        Ok(())
    }
}

impl Solution<String> for Day5 {
    const DAY: usize = 5;

    fn part1(&self, input: &str) -> Option<String> {
        let mut world = input.parse::<Day5World>().unwrap();
        let mut res = String::new();
        for _i in 0..10 {
            res = world.dance();
            //println!("{}: {}", i + 1, res);
            //println!("{}", self);
        }

        Some(res)
    }

    fn part2(&self, input: &str) -> Option<String> {
        let mut world = input.parse::<Day5World>().unwrap();

        let mut shout_counts = HashMap::new();
        let mut round: u128 = 1;
        loop {
            let shouted = world.dance();
            let shouts = shout_counts
                .entry(shouted.clone())
                .and_modify(|c| *c += 1)
                .or_insert(1);

            if *shouts == 2024 {
                // shouted should be a number
                let shouted = shouted.parse::<u128>().unwrap();
                let solution = shouted * round;
                return Some(solution.to_string());
            }
            round += 1;
        }
    }

    fn part3(&self, input: &str) -> Option<String> {
        // let's for now just simulate 10_000_000 steps and find the maximum shouted number
        // not guaranteed to be the correct solution if cycle is longer, also probably really slow
        // cycle detection would certainly be better
        // but hey, this finished in under 2s in release mode, so does it really matter?
        let mut world = input.parse::<Day5World>().unwrap();
        let mut max_shouted: u128 = 0;
        for i in 0..10_000_000 {
            let shouted = world.dance();
            let shouted = shouted.parse::<u128>().unwrap();
            if shouted > max_shouted {
                max_shouted = shouted;
            }
            if i % 1_000_000 == 0 {
                println!("Round {}: max shouted: {}", i, max_shouted);
            }
        }
        Some(max_shouted.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_solution() -> impl Solution<String> {
        Day5
    }

    #[test]
    fn test_part1() {
        let test_input = r#"2 3 4 5
3 4 5 2
4 5 2 3
5 2 3 4"#;

        let solution = get_solution();
        assert_eq!(solution.part1(test_input), Some("2323".to_string()));
    }

    #[test]
    fn test_part2() {
        let test_input = r#"2 3 4 5
6 7 8 9"#;
        let solution = get_solution();
        assert_eq!(solution.part2(test_input), Some("50877075".to_string()));
    }

    /*#[test]
        fn test_part3() {
            let test_input = r#"2 3 4 5
    6 7 8 9"#;
            let mut solution = get_solution();
            assert_eq!(solution.part3(test_input), Some("6584".to_string()));
        }*/
}
