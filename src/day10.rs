use itertools::Itertools;

// Template for new days
use crate::solution::Solution;

pub struct Day10;

fn parse_input(input: &str, offset_x: usize, offset_y: usize) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut columns = vec![Vec::new(); 4];
    let mut rows = vec![Vec::new(); 4];

    for (i, line) in input.lines().skip(offset_y).enumerate().take(8) {
        // first two and last two lines
        if i < 2 || i >= 6 {
            for (col, c) in line.chars().skip(2 + offset_x).take(4).enumerate() {
                columns[col].push(c);
            }
            continue;
        }
        // all others
        rows[i - 2].extend(line.chars().skip(offset_x).filter(|c| *c != '.'));
    }
    (columns, rows)
}

fn runic_word(columns: &[Vec<char>], rows: &[Vec<char>]) -> String {
    let mut result = String::with_capacity(10);
    for y in 0..4 {
        for x in 0..4 {
            let row = &rows[y];
            let col = &columns[x];
            // find only char present in both
            let mut found = false;
            for c in row {
                if col.contains(c) {
                    result.push(*c);
                    found = true;
                    break;
                }
            }
            if !found {
                result.push('?');
            }
        }
    }
    result
}

fn runic_word_power(input: &str) -> usize {
    input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let base_power = c as u8 - 'A' as u8;
            let base_power = base_power as usize + 1;
            base_power * (i + 1)
        })
        .sum()
}

impl Solution<String> for Day10 {
    const DAY: usize = 10;

    fn part1(&self, input: &str) -> Option<String> {
        let (columns, rows) = parse_input(input, 0, 0);
        println!("{:?} {:?}", columns, rows);
        let result = runic_word(&columns, &rows);
        Some(result)
    }

    fn part2(&self, input: &str) -> Option<String> {
        let mut words = Vec::new();
        let width = input.lines().next().unwrap().chars().count();
        let height = input.lines().count();
        for y in 0..((height + 1) / 9) {
            for x in 0..((width + 1) / 9) {
                let offset_x = x * 9;
                let offset_y = y * 9;
                let (columns, rows) = parse_input(input, offset_x, offset_y);
                println!("{:?} {:?}", columns, rows);
                let word = runic_word(&columns, &rows);
                words.push(word);
            }
        }
        println!("words: {:?}", words);
        let result = words
            .into_iter()
            .map(|w| runic_word_power(&w))
            .sum::<usize>()
            .to_string();
        Some(result)
    }

    fn part3(&self, input: &str) -> Option<String> {
        let mut words = Vec::new();
        let width = input.lines().next().unwrap().chars().count();
        let height = input.lines().count();
        for y in 0..((height - 2) / 6) {
            for x in 0..((width - 2) / 6) {
                let offset_x = x * 6;
                let offset_y = y * 6;
                let (columns, rows) = parse_input(input, offset_x, offset_y);
                println!("{:?} {:?}", columns, rows);
                let word = runic_word(&columns, &rows);
                words.push(word);
            }
        }
        println!("words: {:?}", words);
        let result = words
            .into_iter()
            .map(|w| runic_word_power(&w))
            .sum::<usize>()
            .to_string();
        Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_solution() -> impl Solution<String> {
        Day10
    }

    #[test]
    fn test_part1() {
        let test_input = r#"**PCBS**
**RLNW**
BV....PT
CR....HZ
FL....JW
SG....MN
**FTZV**
**GMJH**"#;
        let solution = get_solution();
        assert_eq!(
            solution.part1(test_input),
            Some("PTBVRCZHFLJWGMNS".to_string())
        );
    }

    #[test]
    fn test_part2() {
        let solution = get_solution();
        let test_input = r#"PTBVRCZHFLJWGMNS"#;
        assert_eq!(runic_word_power(test_input), 1851);
        let test_input = r#"**PCBS**
**RLNW**
BV....PT
CR....HZ
FL....JW
SG....MN
**FTZV**
**GMJH**"#;
        assert_eq!(solution.part2(test_input), Some("1851".to_string()));
    }

    #[test]
    fn test_part3() {
        let solution = get_solution();
        let test_input = r#"**XFZB**DCST**
**LWQK**GQJH**
?G....WL....DQ
BS....H?....CN
P?....KJ....TV
NM....Z?....SG
**NSHM**VKWZ**
**PJGV**XFNL**
WQ....?L....YS
FX....DJ....HV
?Y....WM....?J
TJ....YK....LP
**XRTK**BMSP**
**DWZN**GCJV**"#;
        assert_eq!(solution.part3(test_input), Some("3889".to_string()));
    }
}
