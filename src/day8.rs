// Template for new days
use crate::solution::Solution;

pub struct Day8;

struct Pyramid {
    pub rows: Vec<u128>,
    pub sum: u128,
    last_thickness: u128,
    column_heights: Vec<u128>,
}

impl Pyramid {
    pub fn new() -> Self {
        Self {
            rows: vec![1],
            sum: 1,
            last_thickness: 1,
        }
    }

    pub fn add_row(&mut self) {
        let next_row = self.rows[self.rows.len() - 1] + 2;
        self.sum += next_row;
        self.rows.push(next_row);
    }

    pub fn add_row_p2(&mut self, priests: usize) {
        let next_thickness = (self.last_thickness * priests as u128) % 1111;
        let last_width = self.rows[self.rows.len() - 1];
        let next_width = last_width + 2;
        self.sum += next_thickness * next_width;
        self.last_thickness = next_thickness;
        self.rows
            .extend(std::iter::repeat(next_width).take(next_thickness as usize));
    }

    pub fn add_row_p3(&mut self, priests: usize) {
        let next_thickness = ((self.last_thickness * priests as u128) % 10) + 10;
        let last_width = self.rows[self.rows.len() - 1];
        let next_width = last_width + 2;
        self.sum += next_thickness * next_width;
        self.last_thickness = next_thickness;
        self.rows
            .extend(std::iter::repeat(next_width).take(next_thickness as usize));
    }

    pub fn column_heights(&self) -> Vec<u128> {
        let total_width = self.rows[self.rows.len() - 1];
        let middle = total_width / 2;
        let mut heights = vec![0; (total_width as usize)];

        for row in self.rows.iter() {
            // i know I should have done it columnwise in the first place
            // example: row has width 5 -> fill middle, m-1, m+1, m-2, m+2
            let half_width = (row + 1) / 2;
            for i in 0..half_width {
                heights[(middle as usize) + i as usize] += 1;
                if i == 0 {
                    continue;
                }
                heights[(middle as usize) - i as usize] += 1;
            }
        }

        heights
    }

    pub fn empty_blocks(&self, priests: usize) -> u128 {
        let columnwise = self.column_heights();
        let base_width = columnwise.len() as u128;
        let first_line = base_width * priests as u128;
        if columnwise.len() == 1 {
            return 0;
        }
        let correct = columnwise
            .into_iter()
            .skip(1) // outermost blocks are always filled
            .take(base_width as usize - 2)
            .fold(0, |acc, x| ((first_line * x) % 10) + acc);
        correct
    }
}

impl Solution<u128> for Day8 {
    const DAY: usize = 8;

    fn part1(&self, input: &str) -> Option<u128> {
        let parsed: u128 = input.parse().unwrap();
        let mut pyramid = Pyramid::new();
        while pyramid.sum < parsed {
            pyramid.add_row();
        }
        let diff = pyramid.sum - parsed;
        let width = pyramid.rows[pyramid.rows.len() - 1];
        Some(width * diff)
    }

    fn part2(&self, input: &str) -> Option<u128> {
        let parsed: usize = input.parse().unwrap();
        let mut pyramid = Pyramid::new();
        let blocks = 20240000;
        while pyramid.sum < blocks {
            pyramid.add_row_p2(parsed);
        }
        let diff = pyramid.sum - blocks;
        let width = pyramid.rows[pyramid.rows.len() - 1];
        return Some(width * diff);
    }

    fn part3(&self, input: &str) -> Option<u128> {
        let parsed: usize = input.parse().unwrap();
        let mut pyramid = Pyramid::new();
        let blocks = 202400000;

        while (pyramid.sum - pyramid.empty_blocks(parsed)) < blocks {
            pyramid.add_row_p3(parsed);
        }
        let diff = pyramid.sum - blocks - pyramid.empty_blocks(parsed);
        return Some(diff);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_solution() -> impl Solution<u128> {
        Day8
    }

    #[test]
    fn test_part1() {
        let test_input = r#"13"#;
        let solution = get_solution();
        assert_eq!(solution.part1(test_input), Some(21));
        let mut pyramid = Pyramid::new();
        while pyramid.sum < 13 {
            pyramid.add_row();
        }
        let columnwise = pyramid.column_heights();
        //assert_eq!(columnwise, vec![1, 2, 3, 4, 3, 2, 1]);
    }

    #[test]
    fn test_part2() {
        let solution = get_solution();
        let test_input = r#"3"#;
        //assert_eq!(solution.part2(test_input), Some(27));
    }

    #[test]
    fn test_part3() {
        let solution = get_solution();
        let test_input = r#"2"#;
        assert_eq!(solution.part3(test_input), Some(2));
    }
}
