use std::time::Instant;

use solution::Solution;

// mod day1;
// mod day2;
// mod day3;
// mod day4;
mod day5;
pub mod solution;

fn get_input(day: usize, part: usize) -> Option<String> {
    let path = format!("input/{:0>2}_p{}.txt", day, part);
    std::fs::read_to_string(path).ok()
}
fn run_part<T: std::fmt::Debug>(solution: &mut impl Solution<T>, part: usize) {
    let now = Instant::now();
    let day = solution.get_day();
    let input = get_input(day, part);
    if let Some(input) = input {
        let result = match part {
            1 => solution.part1(&input),
            2 => solution.part2(&input),
            3 => solution.part3(&input),
            _ => panic!("Invalid part"),
        };
        if let Some(result) = result {
            println!(
                "Day {} Part {}: {:?} - elapsed: {:?}",
                day,
                part,
                result,
                now.elapsed()
            );
        } else {
            println!("Day {} Part {}: No solution", day, part);
        }
    } else {
        println!("Day {} Part {}: No input", day, part);
    }
}

fn run_day<T: std::fmt::Debug>(mut solution: impl Solution<T>) {
    run_part(&mut solution, 1);
    run_part(&mut solution, 2);
    run_part(&mut solution, 3);
}

fn main() {
    let today = day5::Day5::new();
    run_day(today);
}
