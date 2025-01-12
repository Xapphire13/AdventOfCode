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
        max_hops: u8,
        target_id: &str,
    ) -> Vec<Vec<NodeId>> {
        if max_hops == 0 {
            return vec![];
        }

        let mut results = vec![];

        for neighbor_id in self.neighbors.iter() {
            if path.contains(&neighbor_id) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(neighbor_id.clone());

            if neighbor_id == target_id {
                results.push(new_path);
                continue;
            }

            let neighbor = graph.nodes.get(neighbor_id).unwrap();

            let f = neighbor.find(graph, new_path, max_hops - 1, target_id);
            if !f.is_empty() {
                results.extend(f);
            }
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
                if group.len() >= 3 {
                    Some(NodeGroup { node_ids: group })
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
        String::from("Not implemented")
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
            bb-cc
            cc-aa
            "
        );
        let result = Day23.part1(input);
    }
}
