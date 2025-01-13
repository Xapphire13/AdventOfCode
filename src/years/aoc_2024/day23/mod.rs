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

#[derive(Debug)]
struct NodeGroup {
    node_ids: Vec<NodeId>,
}

impl NodeGroup {
    fn size(&self) -> usize {
        self.node_ids.len()
    }
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

    fn find_groups(&self) -> Vec<NodeGroup> {
        let mut groups = HashSet::new();

        for node in self.nodes.values() {
            let neighbors = &node.neighbors;

            let mut connectivity_map: HashMap<usize, Vec<&Node>> = HashMap::new();

            for neighbor in neighbors.iter().map(|id| self.nodes.get(id).unwrap()) {
                let number_of_shared_neighbors = neighbor
                    .neighbors
                    .iter()
                    .filter(|&id| *id == node.id || neighbors.contains(id))
                    .count();

                connectivity_map
                    .entry(number_of_shared_neighbors)
                    .and_modify(|entry| entry.push(neighbor))
                    .or_insert(vec![neighbor]);
            }

            for (_, neighbor_nodes) in connectivity_map {
                let mut neighbor_nodes = neighbor_nodes;
                while let Some(neighbor) = neighbor_nodes.pop() {
                    let mut remaining_neighbor_nodes = vec![];
                    let neighbors = neighbor_nodes
                        .iter()
                        .filter_map(|&other| {
                            if neighbor.neighbors.contains(&other.id) {
                                Some(other.id.clone())
                            } else {
                                remaining_neighbor_nodes.push(other);
                                None
                            }
                        })
                        .collect_vec();

                    groups.insert(NodeGroup {
                        node_ids: {
                            let mut ids = vec![node.id.clone(), neighbor.id.clone()];
                            ids.extend(neighbors);
                            ids
                        },
                    });

                    neighbor_nodes = remaining_neighbor_nodes;
                }
            }
        }

        groups.into_iter().collect()
    }
}

impl Solution for Day23 {
    fn part1(&self, input: &str) -> String {
        let graph = Graph::parse_input(input);
        let mut result = HashSet::new();

        for group in graph
            .find_groups()
            .into_iter()
            .filter(|group| group.size() >= 3)
        {
            let mut sorted_ids = group
                .node_ids
                .iter()
                .filter(|id| id.starts_with("t"))
                .collect_vec();
            sorted_ids.extend(group.node_ids.iter().filter(|id| !id.starts_with("t")));

            for x in 0..=sorted_ids.len() {
                let first_id = sorted_ids[x];

                if !first_id.starts_with("t") {
                    break;
                }

                for y in (x + 1)..sorted_ids.len() {
                    let second_id = sorted_ids[y];
                    for z in (y + 1)..sorted_ids.len() {
                        let third_id = sorted_ids[z];

                        result.insert(NodeGroup {
                            node_ids: vec![first_id.clone(), second_id.clone(), third_id.clone()],
                        });
                    }
                }
            }
        }

        result.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let graph = Graph::parse_input(input);

        let groups = graph.find_groups();
        let mut largest_group = &groups[0];

        groups.iter().skip(1).for_each(|group| {
            if group.size() > largest_group.size() {
                largest_group = group;
            }
        });

        largest_group.node_ids.iter().sorted().join(",")
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
            t1-t2
            t1-t3
            t1-aa
            t1-bb
            t2-t3
            t2-aa
            t2-bb
            t3-aa
            t3-bb
            aa-bb
            "
        );
        let result = Day23.part1(input);

        assert_eq!(result, "2");
    }
}
