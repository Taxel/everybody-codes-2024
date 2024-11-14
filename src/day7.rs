use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

// Template for new days
use crate::solution::Solution;

pub struct Day7;

fn generate_possible_plans() -> impl Iterator<Item = Vec<Instruction>> {
    let mut plans = HashSet::new();
    generate_plans_recursive(Vec::new(), &mut plans);
    plans.into_iter()
}

fn generate_plans_recursive(current_plan: Vec<Instruction>, plans: &mut HashSet<Vec<Instruction>>) {
    if current_plan.len() == 11 {
        plans.insert(current_plan);
        return;
    }
    let (add, subtract, maintain) = current_plan.iter().fold(
        (0, 0, 0),
        |(add, subtract, maintain), action| match action {
            Instruction::AddOne => (add + 1, subtract, maintain),
            Instruction::SubtractOne => (add, subtract + 1, maintain),
            Instruction::Maintain => (add, subtract, maintain + 1),
        },
    );
    if add < 5 {
        let mut new_plan = current_plan.clone();
        new_plan.push(Instruction::AddOne);
        generate_plans_recursive(new_plan, plans);
    }
    if subtract < 3 {
        let mut new_plan = current_plan.clone();
        new_plan.push(Instruction::SubtractOne);
        generate_plans_recursive(new_plan, plans);
    }
    if maintain < 3 {
        let mut new_plan = current_plan.clone();
        new_plan.push(Instruction::Maintain);
        generate_plans_recursive(new_plan, plans);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn rotate_clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn rotate_counterclockwise(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    pub fn try_march(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Self::North => {
                if y == height - 1 {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Self::East => {
                if x == width - 1 {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            Self::South => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Self::West => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Instruction {
    AddOne,
    SubtractOne,
    Maintain,
}

impl From<char> for Instruction {
    fn from(c: char) -> Self {
        match c {
            '+' => Self::AddOne,
            '-' => Self::SubtractOne,
            '=' => Self::Maintain,
            'S' => Self::Maintain,
            _ => panic!("Invalid instruction"),
        }
    }
}
struct Chariot {
    pub name: String,
    pub power: usize,
    pub essence_collected: usize,
    instruction_idx: usize,
    plan: Vec<Instruction>,
}

impl Chariot {
    pub fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::AddOne => self.power += 1,
            Instruction::SubtractOne => self.power = self.power.saturating_sub(1),
            Instruction::Maintain => (),
        }
        self.essence_collected += self.power;
        self.instruction_idx = (self.instruction_idx + 1) % self.plan.len();
    }
}

impl FromStr for Chariot {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let name = parts.next().unwrap();
        let plan = parts.next().unwrap().split(',').map(|s| match s {
            "+" => Instruction::AddOne,
            "-" => Instruction::SubtractOne,
            "=" => Instruction::Maintain,
            _ => panic!("Invalid instruction"),
        });
        Ok(Self {
            name: name.to_string(),
            power: 10,
            essence_collected: 0,
            instruction_idx: 0,
            plan: plan.collect(),
        })
    }
}

struct Day7World {
    pub chariots: Vec<Chariot>,
    current_segment: usize,
    track: Option<Vec<Instruction>>,
}

impl Day7World {
    pub fn run_step(&mut self) {
        let track_action = if let Some(track) = &self.track {
            track[self.current_segment]
        } else {
            Instruction::Maintain
        };
        for chariot in self.chariots.iter_mut() {
            let to_execute = match track_action {
                Instruction::AddOne => Instruction::AddOne,
                Instruction::SubtractOne => Instruction::SubtractOne,
                Instruction::Maintain => chariot.plan[chariot.instruction_idx],
            };
            chariot.apply_instruction(to_execute);
        }
        self.current_segment += 1;
        if let Some(track) = &self.track {
            self.current_segment %= track.len();
        }
    }

    pub fn run_steps(&mut self, amount: usize) {
        for _ in 0..amount {
            self.run_step();
        }
    }

    pub fn run_loop(&mut self) {
        let len = self.track.as_ref().unwrap().len();
        for _ in 0..len {
            self.run_step();
        }
    }

    fn track_from_ascii(ascii_track: &str) -> Vec<Instruction> {
        // first, we collect the chars into a vec (for easier indexing) and determine the dimensions of this 2d grid
        let mut instructions: Vec<Option<Instruction>> = Vec::with_capacity(ascii_track.len());
        let width = ascii_track.lines().map(|l| l.len()).max().unwrap();
        let mut height = 0;
        let mut track = Vec::new();
        for line in ascii_track.lines() {
            height += 1;
            let to_pad = width - line.len();
            instructions.extend(line.chars().map(|c| match c {
                ' ' => None,
                _ => Some(c.into()),
            }));
            instructions.extend(std::iter::repeat(None).take(to_pad));
        }
        let vec_idx = |(x, y): (usize, usize)| -> usize { ((height - y - 1) * width) + x };
        let get_next_pos = |(x, y): (usize, usize), dir: Direction| -> Option<(usize, usize)> {
            dir.try_march(x, y, width, height)
        };
        // we always start at the top left corner (0,height-1), looking east
        let mut current_pos = (0, height - 1);
        let mut current_dir = Direction::East;
        current_pos = get_next_pos(current_pos, current_dir).unwrap();

        while current_pos != (0, height - 1) {
            let idx = vec_idx(current_pos);
            track.push(instructions[idx].unwrap());
            // if possible, we turn right
            let next_dir = current_dir.rotate_clockwise();
            let next_pos = get_next_pos(current_pos, next_dir);
            if let Some(next_pos) = next_pos {
                let idx = vec_idx(next_pos);
                if instructions[idx].is_some() {
                    current_dir = next_dir;
                    current_pos = next_pos;
                    continue;
                }
            }
            // then we try to go straight
            let next_pos = get_next_pos(current_pos, current_dir);
            if let Some(next_pos) = next_pos {
                let idx = vec_idx(next_pos);
                if instructions[idx].is_some() {
                    current_pos = next_pos;
                    continue;
                }
            }
            // finally, we turn left
            let next_dir = current_dir.rotate_counterclockwise();
            let next_pos = get_next_pos(current_pos, next_dir);
            if let Some(next_pos) = next_pos {
                let idx = vec_idx(next_pos);
                if instructions[idx].is_some() {
                    current_dir = next_dir;
                    current_pos = next_pos;
                    continue;
                } else {
                    panic!("No way out at {:?}", current_pos);
                }
            }
        }
        // lastly, add the start
        track.push('S'.into());

        track
    }
}

impl FromStr for Day7World {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("\n\n");
        let chariots = split
            .next()
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();

        let track = split.next().map(Self::track_from_ascii);
        Ok(Self {
            chariots,
            current_segment: 0,
            track,
        })
    }
}

impl Solution<String> for Day7 {
    const DAY: usize = 7;

    fn part1(&self, input: &str) -> Option<String> {
        let mut world = input.parse::<Day7World>().unwrap();
        world.run_steps(10);
        let result = world
            .chariots
            .iter()
            .map(|c| (c.name.clone(), c.essence_collected))
            .sorted_by(|a, b| b.1.cmp(&a.1))
            .collect_vec();
        Some(result.into_iter().map(|(name, _)| name).join(""))
    }

    fn part2(&self, input: &str) -> Option<String> {
        let mut world = input.parse::<Day7World>().unwrap();
        for _ in 0..10 {
            world.run_loop();
        }
        let result = world
            .chariots
            .iter()
            .map(|c| (c.name.clone(), c.essence_collected))
            .sorted_by(|a, b| b.1.cmp(&a.1))
            .collect_vec();
        Some(result.into_iter().map(|(name, _)| name).join(""))
    }

    fn part3(&self, input: &str) -> Option<String> {
        let mut world = input.parse::<Day7World>().unwrap();
        world
            .chariots
            .extend(generate_possible_plans().map(|plan| Chariot {
                name: "S".to_string(),
                power: 10,
                essence_collected: 0,
                instruction_idx: 0,
                plan,
            }));

        println!("Starting loop. Total chariots: {}", world.chariots.len());
        for _ in 0..2024 {
            world.run_loop();
        }
        println!("Loop done");
        // how many did the first strategy collect?
        let to_beat = world.chariots[0].essence_collected;
        let strats_beating_1 = world
            .chariots
            .iter()
            .filter(|c| c.essence_collected > to_beat)
            .count();
        Some(strats_beating_1.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_solution() -> impl Solution<String> {
        Day7
    }

    #[test]
    fn test_part1() {
        let test_input = r#"A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+"#;
        let solution = get_solution();
        assert_eq!(solution.part1(test_input), Some("BDCA".to_string()));
    }

    #[test]
    fn test_part2() {
        let solution = get_solution();
        let test_input = r#"A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+

S+===
-   +
=+=-+"#;
        assert_eq!(solution.part2(test_input), Some("DCBA".to_string()));
    }

    #[test]
    fn test_part3() {
        let test_input = r#"A:=,+,=,+,+,-,+,=,+,-,-

S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
- + +   + =   =     =      =   == = - -     - =  =         =-=        -
= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=
= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =
+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
-               = = = =   +  +  ==+ = = +   =        ++    =          -
-               = + + =   +  -  = + = = +   =        +     =          -
--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-"#;
        let solution = get_solution();
        assert_eq!(solution.part3(test_input), None);
    }
}
