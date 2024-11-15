#![allow(unreachable_code)]
use crate::solution::Solution;
use std::collections::VecDeque;

pub struct Day2;

fn get_match_mask(line: &str, words: &[&str], is_ring: bool) -> Vec<bool> {
    // vecdeques can be easily rotated
    let mut mask = VecDeque::from_iter(std::iter::repeat(false).take(line.len()));
    let mut line = line.chars().collect::<VecDeque<_>>();

    if !is_ring {
        // add one more char that's unused to line
        line.push_back('$');
        mask.push_back(false);
    }

    for word in words {
        // idea: rotate through line and mask simultaneously, always checking if the first chars are correct
        let word_len = word.len();
        for _i in 0..line.len() {
            if line.iter().take(word_len).copied().eq(word.chars()) {
                // mark first word_len entries in mask as found

                for item in mask.iter_mut().take(word_len) {
                    *item = true;
                }
            } else if line.iter().take(word_len).rev().copied().eq(word.chars()) {
                // reversed word found

                for item in mask.iter_mut().take(word_len) {
                    *item = true;
                }
            }
            // rotate
            mask.rotate_left(1);
            line.rotate_left(1);
        }
    }

    mask.into_iter().collect()
}

impl Solution<i32> for Day2 {
    const DAY: usize = 2;

    fn part1(&self, input: &str) -> Option<i32> {
        let mut lines = input.lines();
        let words = lines
            .next()
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .collect::<Vec<&str>>();
        // skip next line
        lines.next();
        let sentence = lines.next().unwrap();
        let mut count = 0;
        for word in words {
            let this_word_count = sentence.matches(word).count();

            count += this_word_count;
        }
        Some(count as i32)
    }

    fn part2(&self, input: &str) -> Option<i32> {
        let mut lines = input.lines();
        let words = lines
            .next()
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .collect::<Vec<&str>>();
        // skip next line
        lines.next();
        let mut count = 0;
        for sentence in lines {
            let rune_mask = get_match_mask(sentence, &words, false);

            count += rune_mask.iter().filter(|&&x| x).count();
        }
        Some(count as i32)
    }

    fn part3(&self, input: &str) -> Option<i32> {
        let mut lines = input.lines();
        let words = lines
            .next()
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .collect::<Vec<&str>>();
        // skip next line
        lines.next();

        // define bool matrix
        let lines: Vec<&str> = lines.collect();
        let mut mask = vec![vec![false; lines[0].len()]; lines.len()];
        for (i, line) in lines.iter().enumerate() {
            let rune_mask = get_match_mask(line, &words, true);
            for (j, &x) in rune_mask.iter().enumerate() {
                mask[i][j] = x;
            }
        }
        // vertical check
        for x in 0..lines[0].len() {
            let mut line = String::with_capacity(lines.len());
            for str_line in &lines {
                line.push(str_line.chars().nth(x).unwrap());
            }
            let rune_mask = get_match_mask(&line, &words, false);
            for (y, is_rune) in rune_mask.iter().enumerate().take(lines.len()) {
                if *is_rune {
                    mask[y][x] = true;
                }
            }
        }
        Some(mask.iter().flatten().filter(|&&x| x).count() as i32)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let day = Day2;
        let input = r#"WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE"#;
        assert_eq!(day.part1(input), Some(4));
    }

    #[test]
    fn test_part2() {
        return;
        let day = Day2;
        let input = r#"WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END"#;
        assert_eq!(day.part2(input), Some(37));
    }

    #[test]
    fn test_part3() {
        let day = Day2;
        let input = r#"WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL"#;
        assert_eq!(day.part3(input), Some(10));
    }
}
