use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use itertools::Itertools;
use regex::Regex;

use crate::aoc_solution::Solution;

pub struct Day23;

type NodeId = String;

#[derive(Debug)]
struct Node {
    id: NodeId,
    neighbors: HashSet<NodeId>,
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for Node {}

impl Node {
    fn find(
        &self,
        graph: &Graph,
        path: Vec<NodeId>,
        max_hops: usize,
        target_id: &str,
    ) -> Vec<Vec<NodeId>> {
        if path.len() >= 1 && self.id == target_id {
            let mut new_path = path.clone();
            if !new_path.contains(&self.id) {
                new_path.push(self.id.clone());
            }
            return vec![new_path];
        }

        if max_hops == 0 {
            return vec![];
        }

        if path.contains(&self.id) {
            return vec![];
        }

        let mut new_path = path.clone();
        new_path.push(self.id.clone());

        let mut results = vec![];

        for neighbor_id in self.neighbors.iter() {
            let neighbor = graph.nodes.get(neighbor_id).unwrap();

            results.extend(neighbor.find(graph, new_path.clone(), max_hops - 1, target_id));
        }

        results
    }
}

#[derive(Debug)]
struct NodeGroup {
    node_ids: Vec<NodeId>,
}

impl Hash for NodeGroup {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.node_ids.iter().sorted().join(":").hash(state);
    }
}

impl PartialEq for NodeGroup {
    fn eq(&self, other: &Self) -> bool {
        self.node_ids.iter().sorted().join(":") == other.node_ids.iter().sorted().join(":")
    }
}

impl Eq for NodeGroup {}

struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    fn parse_input(input: &str) -> Graph {
        let mut nodes = HashMap::new();
        let regex = Regex::new("(..)-(..)").unwrap();

        for line in input.trim().lines() {
            let (_, [node1_id, node2_id]) = regex.captures(line).unwrap().extract();

            nodes
                .entry(node1_id.to_string())
                .and_modify(|node: &mut Node| {
                    node.neighbors.insert(node2_id.to_string());
                })
                .or_insert(Node {
                    id: node1_id.to_string(),
                    neighbors: HashSet::from([node2_id.to_string()]),
                });

            nodes
                .entry(node2_id.to_string())
                .and_modify(|node: &mut Node| {
                    node.neighbors.insert(node1_id.to_string());
                })
                .or_insert(Node {
                    id: node2_id.to_string(),
                    neighbors: HashSet::from([node1_id.to_string()]),
                });
        }

        Graph { nodes }
    }
}

impl Solution for Day23 {
    fn part1(&self, input: &str) -> String {
        let graph = Graph::parse_input(input);

        let groups: HashSet<NodeGroup> = graph
            .nodes
            .values()
            .filter_map(|node| {
                let result = node.find(&graph, vec![], 3, node.id.as_str());

                if result.is_empty() {
                    None
                } else {
                    Some(result)
                }
            })
            .flatten()
            .filter_map(|group| {
                if group.len() == 3 {
                    Some(NodeGroup {
                        node_ids: group.iter().cloned().collect(),
                    })
                } else {
                    None
                }
            })
            .filter(|group| {
                group
                    .node_ids
                    .iter()
                    .find(|group_id| group_id.starts_with("t"))
                    .is_some()
            })
            .collect();

        groups.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let graph = Graph::parse_input(input);

        let groups: HashSet<NodeGroup> = graph
            .nodes
            .values()
            .enumerate()
            .filter_map(|(i, node)| {
                println!(
                    "{}/{} - {} neighbors",
                    i + 1,
                    graph.nodes.len(),
                    node.neighbors.len()
                );
                let result = node.find(&graph, vec![], node.neighbors.len() + 1, node.id.as_str());

                if result.is_empty() {
                    None
                } else {
                    Some(result)
                }
            })
            .flatten()
            .filter_map(|group| {
                if group.len() >= 3 {
                    Some(NodeGroup {
                        node_ids: group.iter().cloned().collect(),
                    })
                } else {
                    None
                }
            })
            .collect();

        groups.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use dedent::dedent;

    use super::*;

    #[test]
    fn test_day23() {
        let input = dedent!(
            "
            aa-bb
            cc-aa
            "
        );
        let result = Day23.part1(input);

        assert_eq!(result, "0");
    }
}
