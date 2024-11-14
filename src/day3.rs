use crate::solution::Solution;

pub struct Day3;

/// Tries to dig the level provided as depth, return true if successful
fn carve_deeper(
    grid: &mut Vec<u16>,
    depth: u16,
    width: usize,
    height: usize,
    respect_diagonals: bool,
) -> bool {
    let mut did_something = false;
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            // is this at depth - 1?
            if grid[idx] != depth - 1 {
                continue;
            }
            // map is padded with 0s, so for all pixels at the edge of the map, we can skip them
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                continue;
            }
            // are all neighbors at least depth - 1?
            let mut all_neighbors = true;
            if x > 0 && grid[y * width + x - 1] < depth - 1 {
                all_neighbors = false;
            }
            if x < width - 1 && grid[y * width + x + 1] < depth - 1 {
                all_neighbors = false;
            }
            if y > 0 && grid[(y - 1) * width + x] < depth - 1 {
                all_neighbors = false;
            }
            if y < height - 1 && grid[(y + 1) * width + x] < depth - 1 {
                all_neighbors = false;
            }
            if respect_diagonals {
                // also check diagonals
                if x > 0 && y > 0 && grid[(y - 1) * width + x - 1] < depth - 1 {
                    all_neighbors = false;
                }
                if x < width - 1 && y > 0 && grid[(y - 1) * width + x + 1] < depth - 1 {
                    all_neighbors = false;
                }
                if x > 0 && y < height - 1 && grid[(y + 1) * width + x - 1] < depth - 1 {
                    all_neighbors = false;
                }
                if x < width - 1 && y < height - 1 && grid[(y + 1) * width + x + 1] < depth - 1 {
                    all_neighbors = false;
                }
            }
            if all_neighbors {
                grid[idx] = depth;
                did_something = true;
            }
        }
    }
    did_something
}

impl Solution<i32> for Day3 {
    const DAY: usize = 3;

    fn part1(&self, input: &str) -> Option<i32> {
        let lines = input.lines().collect::<Vec<&str>>();
        let width = lines[0].len();
        let height = lines.len();
        let mut grid = vec![0; width * height];
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                if lines[y].chars().nth(x).unwrap() == '#' {
                    grid[idx] = 1;
                }
            }
        }
        let mut depth = 2;
        while carve_deeper(&mut grid, depth, width, height, false) {
            depth += 1;
        }
        Some(grid.into_iter().map(|n| n as i32).sum())
    }

    fn part2(&self, input: &str) -> Option<i32> {
        // identical to part1
        self.part1(input)
    }

    fn part3(&self, input: &str) -> Option<i32> {
        let lines = input.lines().collect::<Vec<&str>>();
        let width = lines[0].len();
        let height = lines.len();
        let mut grid = vec![0; width * height];
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                if lines[y].chars().nth(x).unwrap() == '#' {
                    grid[idx] = 1;
                }
            }
        }
        let mut depth = 2;
        while carve_deeper(&mut grid, depth, width, height, true) {
            depth += 1;
        }
        Some(grid.into_iter().map(|n| n as i32).sum())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"..........
..###.##..
...####...
..######..
..######..
...####...
.........."#;
        let day = Day3;
        assert_eq!(day.part1(input), Some(35));
    }

    #[test]
    fn test_part3() {
        let input = r#"..........
..###.##..
...####...
..######..
..######..
...####...
.........."#;
        let day = Day3;
        assert_eq!(day.part3(input), Some(29));
    }
}
