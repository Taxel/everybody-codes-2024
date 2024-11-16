use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
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
    let mut precursors = HashMap::new();
    while let Some(node) = queue.pop_front() {
        if node == goal_node {
            return Some(build_path_from_precursors::<G>(&goal_node, &precursors));
        }
        for neighbor in graph.neighbors(&node) {
            if !visited.contains(&neighbor) {
                queue.push_back(neighbor.clone());
                precursors.insert(neighbor.clone(), node.clone());
                visited.insert(neighbor);
            }
        }
    }
    return None;
}
/*
pub fn dijkstra<G: DirectedGraph>(
    graph: &mut G,
    start_node: G::Node,
    goal_node: G::Node,
) -> Option<impl Iterator<Item = G::Node>> {
    let mut queue = BinaryHeap::from([start_node]);
    let mut distances = HashMap::from([(start_node.clone(), 0)]);
    //let mut precursors = HashMap::new();
    while let Some(node) = queue.pop() {}
    todo!()
    //build_path_from_precursors(&goal_node, &precursors)
} */

pub trait Graph {
    /// Node should be easy to clone etc, we do this a lot
    type Node: Clone + Eq + Hash;

    fn neighbors(&mut self, n: &Self::Node) -> impl Iterator<Item = Self::Node>;
}

pub trait DirectedGraph {
    /// Node should be easy to clone etc, we do this a lot
    /// it also needs to be Ord for a directed graph and Ord must return in reverse order (because std's BinaryHeap is a MaxHeap)
    type Node: Clone + Eq + Hash + Ord;
    type Distance: Copy + Clone + Eq + Hash;

    fn neighbors_with_distance(
        &mut self,
        n: &Self::Node,
    ) -> impl Iterator<Item = (Self::Node, Self::Distance)>;
}

impl<G: DirectedGraph> Graph for G {
    type Node = G::Node;

    fn neighbors(&mut self, n: &Self::Node) -> impl Iterator<Item = Self::Node> {
        self.neighbors_with_distance(n).map(|(n, d)| n)
    }
}
