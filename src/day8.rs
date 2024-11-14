// Template for new days
use crate::solution::Solution;

pub struct Day8;

struct Pyramid {
    pub rows: Vec<u128>,
    pub sum: u128,
    last_thickness: u128,
    // because of the symmetry, we only keep the right half of the heights
    column_heights: Vec<u128>,
}

impl Pyramid {
    pub fn new() -> Self {
        Self {
            rows: vec![1],
            sum: 1,
            last_thickness: 1,
            column_heights: vec![1],
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
        self.column_heights.push(0);
        self.column_heights
            .iter_mut()
            .for_each(|x| *x += next_thickness);
        self.rows
            .extend(std::iter::repeat(next_width).take(next_thickness as usize));
    }

    pub fn column_heights(&self) -> Vec<u128> {
        let width = self.rows[self.rows.len() - 1] as usize;
        let mut columnwise = Vec::with_capacity(width as usize);
        let half_width = width / 2;
        for i in 0..width {
            let idx = (i as i32 - half_width as i32).abs() as usize;
            columnwise.push(self.column_heights[idx])
        }
        columnwise
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
        //assert_eq!(solution.part3(test_input), Some(2));
    }
}
