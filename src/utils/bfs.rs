use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

fn build_path_from_precursors<G: Graph>(
    goal_node: &G::Node,
    precursors: &HashMap<G::Node, G::Node>,
) -> impl Iterator<Item = G::Node> {
    let mut path = vec![goal_node.clone()];
    let mut current = goal_node.clone();
    while let Some(node) = precursors.get(&current) {
        path.push(node.clone());
        current = node.clone();
    }
    path.into_iter().rev()
}

/// returns the path to the goal
pub fn bfs<G: Graph>(
    graph: &mut G,
    start_node: G::Node,
    goal_node: G::Node,
) -> Option<impl Iterator<Item = G::Node>> {
    let mut queue = VecDeque::from([start_node.clone()]);
    let mut visited = HashSet::from([start_node.clone()]);
    let mut precursor = HashMap::new();
    while let Some(node) = queue.pop_front() {
        if node == goal_node {
            return Some(build_path_from_precursors::<G>(&goal_node, &precursor));
        }
        for neighbor in graph.neighbors(&node) {
            if !visited.contains(&neighbor) {
                queue.push_back(neighbor.clone());
                precursor.insert(neighbor.clone(), node.clone());
                visited.insert(neighbor);
            }
        }
    }
    return None;
}

pub trait Graph {
    /// Node should be easy to clone etc, we do this a lot
    type Node: Clone + Eq + Hash;

    fn neighbors(&mut self, n: &Self::Node) -> impl Iterator<Item = Self::Node>;
}
