// Template for new days
use crate::solution::Solution;

pub struct Day6;

impl Solution<i64> for Day6 {
    const DAY: usize = 6;

    fn part1(&self, input: &str) -> Option<i64> {
        todo!()
    }

    fn part2(&self, input: &str) -> Option<i64> {
        todo!()
    }

    fn part3(&self, input: &str) -> Option<i64> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_solution() -> impl Solution<i64> {
        Day6
    }

    #[test]
    fn test_part1() {
        let test_input = r#""#;
        let solution = get_solution();
        assert_eq!(solution.part1(test_input), None);
    }

    #[test]
    fn test_part2() {
        let solution = get_solution();
        let test_input = r#""#;
        assert_eq!(solution.part2(test_input), None);
    }

    #[test]
    fn test_part3() {
        let solution = get_solution();
        let test_input = r#""#;
        assert_eq!(solution.part3(test_input), None);
    }
}
