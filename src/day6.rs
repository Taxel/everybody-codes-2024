use std::{
    collections::{HashMap, HashSet},
    path,
    str::FromStr,
};

use itertools::Itertools;

// Template for new days
use crate::solution::{self, Solution};

pub struct Day6;

// yay, trees in rust \s
#[derive(Debug, PartialEq, Clone)]
struct TreeNode<'a> {
    name: &'a str,
    children: Vec<&'a str>,
    parent: Option<&'a str>,
}

impl<'a> From<&'a str> for TreeNode<'a> {
    fn from(s: &'a str) -> Self {
        // gets a single line
        let mut parts = s.split(':');
        let name = parts.next().unwrap();
        let children = parts.next().unwrap().split(',').collect();
        TreeNode {
            name,
            children,
            parent: None,
        }
    }
}

struct Day6World<'a> {
    pub tree: HashMap<&'a str, TreeNode<'a>>,
}

impl<'a> Day6World<'a> {
    pub fn new() -> Self {
        Self {
            tree: HashMap::new(),
        }
    }

    pub fn populate_parents(&mut self) {
        let mut parents = Vec::new();
        for (name, node) in self.tree.iter() {
            for child in node.children.iter() {
                if child == &"@" {
                    continue;
                }
                parents.push((*child, *name));
            }
        }

        // sort out children with more than one parent. these are probably the bugs/ants
        let mut child_to_parent = HashMap::new();
        for (child, parent) in parents {
            let entry = child_to_parent.entry(child).or_insert(Vec::new());
            entry.push(parent);
        }

        for (child, parents) in child_to_parent {
            if parents.len() > 1 {
                // this is not an actual branch
                self.tree.remove(child);
                continue;
            }
            if child == "@" {
                continue;
            }
            let parent = parents[0];
            if let Some(child) = self.tree.get_mut(child) {
                if child.parent.is_some() {
                    panic!("Child already has a parent");
                }
                child.parent = Some(parent);
            }
        }
    }

    pub fn depth_of(&self, node: &str) -> usize {
        let mut depth = 0;
        let mut current = node;
        while let Some(parent) = self.tree.get(current).unwrap().parent {
            depth += 1;
            current = parent;
        }
        depth
    }

    pub fn path_to(&'a self, node: &'a str) -> Vec<&'a str> {
        let mut path = Vec::new();
        let mut current = node;
        while let Some(parent) = self.tree.get(current).unwrap().parent {
            path.push(current);
            current = parent;
        }
        // push root
        path.push(current);
        path.into_iter().rev().collect()
    }

    pub fn bfs_find_singular_solution(&'a self, start: &'a str) -> Option<Vec<&'a str>> {
        let mut queue = vec![start];
        let mut visited = HashSet::new();
        let mut paths = HashMap::new();

        while let Some(node) = queue.pop() {
            if let Some(tree_node) = self.tree.get(node) {
                let mut added_apple_path = false;
                for child in &tree_node.children {
                    if child == &"@" {
                        if added_apple_path {
                            continue;
                        }
                        let mut path = self.path_to(node);
                        let paths = paths.entry(path.len()).or_insert(Vec::new());
                        if paths.len() < 2 {
                            path.push("@");
                            paths.push(path);
                        }
                        added_apple_path = true;
                        continue;
                    }
                    if self.tree.contains_key(child) && !visited.contains(&child) {
                        visited.insert(child);
                        queue.push(child);
                    }
                }
            } else {
                //println!("Ant or Bug: {:?}", node);
            }
        }

        // print paths, sorted by length
        for (len, paths) in paths.iter().sorted_by_key(|(len, _)| *len) {
            println!("{}: {} paths", len, paths.len());
        }
        // find entry with exactly one path
        paths
            .into_iter()
            .find(|(_, paths)| paths.len() == 1)
            .map(|(_, paths)| paths[0].clone())
    }
}

impl<'a> From<&'a str> for Day6World<'a> {
    fn from(s: &'a str) -> Self {
        let mut world = Day6World::new();
        for line in s.lines() {
            let node: TreeNode = line.into();
            world.tree.insert(node.name, node);
        }
        world.populate_parents();
        world
    }
}

impl Solution<String> for Day6 {
    const DAY: usize = 6;

    fn part1(&self, input: &str) -> Option<String> {
        let world: Day6World<'_> = input.into();
        let solution = world.bfs_find_singular_solution("RR");
        Some(solution.unwrap().join(""))
    }

    fn part2(&self, input: &str) -> Option<String> {
        let world: Day6World<'_> = input.into();
        let final_path = world.bfs_find_singular_solution("RR");
        Some(
            final_path
                .unwrap()
                .into_iter()
                .map(|s| s.chars().next().unwrap())
                .join(""),
        )
    }

    fn part3(&self, input: &str) -> Option<String> {
        self.part2(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_solution() -> impl Solution<String> {
        Day6
    }

    #[test]
    fn test_part1() {
        let test_input = r#"RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@"#;
        let solution = get_solution();
        assert_eq!(solution.part1(test_input), Some("RRB@".to_string()));
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
