use itertools::Itertools;

use crate::solution::Solution;

pub struct Day4;

impl Solution<i32> for Day4 {
    const DAY: usize = 4;

    fn part1(&self, input: &str) -> Option<i32> {
        let nums = input
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let min = *nums.iter().min().unwrap();
        Some(nums.into_iter().map(|n| n - min).sum())
    }

    fn part2(&self, input: &str) -> Option<i32> {
        self.part1(input)
    }

    fn part3(&self, input: &str) -> Option<i32> {
        let nums = input
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .sorted()
            .collect::<Vec<_>>();

        let mut min = i32::MAX;
        let median = nums[nums.len() / 2];
        let eval = |n: i32| nums.iter().map(|num| (n - num).abs()).sum::<i32>();

        let at_median = eval(median);
        let below = eval(median - 1);
        let above = eval(median + 1);

        if below < at_median {
            // go down until minimum found
            min = below;
            let mut current = median - 2;
            loop {
                let result = eval(current);
                if result < min {
                    min = result;
                    current -= 1;
                } else {
                    break;
                }
            }
        } else if above < at_median {
            // go up until minimum found
            min = above;
            let mut current = median + 2;
            loop {
                let result = eval(current);
                if result < min {
                    min = result;
                    current += 1;
                } else {
                    break;
                }
            }
        } else {
            // median must be minimum
            min = at_median;
        }

        Some(min)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p3() {
        let d4 = Day4;
        let input = r#"2
4
5
6
8"#;
        assert_eq!(d4.part3(input), Some(8))
    }
}
