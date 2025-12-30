use std::{collections::HashSet, hash::Hash};

use shared::{IPoint3D, Solution};

pub struct Day8;

#[derive(Debug, PartialEq, Eq, Hash)]
struct JunctionBox {
    id: usize,
    position: IPoint3D,
}

struct CircuitResult {
    connected_circuits: Vec<HashSet<usize>>,
    last_connected_pair: (usize, usize),
}

#[derive(Debug)]
struct Problem {
    junctions: Vec<JunctionBox>,
}

impl Problem {
    fn new(input: &str) -> Self {
        let junctions: Vec<_> = input
            .trim()
            .lines()
            .enumerate()
            .map(|(id, line)| {
                let mut split = line.split(",");

                JunctionBox {
                    id,
                    position: IPoint3D::new(
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap(),
                    ),
                }
            })
            .collect();

        Self { junctions }
    }

    fn connected_circuits(&self, number_of_connections: Option<usize>) -> CircuitResult {
        let distances = self.compute_sorted_distances();
        self.build_circuits(distances, number_of_connections)
    }

    fn compute_sorted_distances(&self) -> Vec<((usize, usize), f64)> {
        let mut distances = vec![];

        for i in 0..self.junctions.len() {
            for j in (i + 1)..self.junctions.len() {
                let junction = &self.junctions[i];
                let other = &self.junctions[j];

                let distance = junction.position.distance(&other.position);

                distances.push(((i, j), distance));
            }
        }

        distances.sort_by(|(_, lhs), (_, rhs)| lhs.total_cmp(rhs));
        distances
    }

    fn build_circuits(
        &self,
        distances: Vec<((usize, usize), f64)>,
        number_of_connections: Option<usize>,
    ) -> CircuitResult {
        let mut connected_circuits: Vec<HashSet<usize>> = vec![];
        let mut last_connected_pair = (0, 0);

        for ((left_id, right_id), _) in distances
            .iter()
            .take(number_of_connections.unwrap_or(distances.len()))
        {
            let mut left_circuit =
                Problem::find_or_create_circuit(&mut connected_circuits, *left_id);
            let right_circuit = Problem::find_or_create_circuit(&mut connected_circuits, *right_id);

            left_circuit.extend(right_circuit);
            connected_circuits.push(left_circuit);
            last_connected_pair = (*left_id, *right_id);

            // All circuits connect into one
            if connected_circuits.len() == 1 && connected_circuits[0].len() == self.junctions.len()
            {
                break;
            }
        }

        CircuitResult {
            connected_circuits,
            last_connected_pair,
        }
    }

    fn find_or_create_circuit(circuits: &mut Vec<HashSet<usize>>, id: usize) -> HashSet<usize> {
        circuits
            .iter()
            .position(|c| c.contains(&id))
            .map(|idx| circuits.swap_remove(idx))
            .unwrap_or_else(|| HashSet::from([id]))
    }
}

impl Solution for Day8 {
    fn part1(&self, input: &str) -> String {
        let problem = Problem::new(input);
        let CircuitResult {
            mut connected_circuits,
            ..
        } = problem.connected_circuits(Some(1000));

        connected_circuits.sort_by_key(|c| c.len());
        connected_circuits
            .iter()
            .rev()
            .take(3)
            .map(|c| c.len())
            .reduce(|acc, e| acc * e)
            .unwrap_or(0)
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let problem = Problem::new(input);
        let CircuitResult {
            last_connected_pair: (left_id, right_id),
            ..
        } = problem.connected_circuits(None);

        let left_x = problem.junctions[left_id].position.x as i64;
        let right_x = problem.junctions[right_id].position.x as i64;

        (left_x * right_x).to_string()
    }
}
