use std::{collections::HashSet, hash::Hash};

use shared::{IPoint3D, Solution};

pub struct Day8;

#[derive(Debug, PartialEq, Eq, Hash)]
struct JunctionBox {
    id: usize,
    position: IPoint3D,
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

    fn circuits(
        &self,
        number_of_connections: Option<usize>,
    ) -> (Vec<HashSet<usize>>, (usize, usize)) {
        let mut circuits: Vec<HashSet<usize>> = vec![];
        let mut junction_distances = vec![];

        for junction in &self.junctions {
            for other in &self.junctions {
                if junction == other {
                    continue;
                }

                let distance = junction.position.distance(&other.position);

                let &first_id = [junction.id, other.id].iter().min().unwrap();
                let &second_id = [junction.id, other.id].iter().max().unwrap();

                junction_distances.push(((first_id, second_id), distance));
            }
        }
        junction_distances.sort_by(|(_, lhs), (_, rhs)| lhs.total_cmp(rhs));
        junction_distances.dedup_by_key(|d| d.0);
        let mut last_connection = (0, 0);

        for ((left_id, right_id), _) in junction_distances
            .iter()
            .take(number_of_connections.unwrap_or(junction_distances.len()))
        {
            let mut left_circuit = match circuits
                .iter()
                .enumerate()
                .find(|(_, c)| c.contains(left_id))
            {
                Some((idx, _)) => circuits.swap_remove(idx),
                None => HashSet::from([*left_id]),
            };
            let right_circuit = match circuits
                .iter()
                .enumerate()
                .find(|(_, c)| c.contains(right_id))
            {
                Some((idx, _)) => circuits.swap_remove(idx),
                None => HashSet::from([*right_id]),
            };

            left_circuit.extend(right_circuit);
            circuits.push(left_circuit);
            last_connection = (*left_id, *right_id);

            // All circuits connect into one
            if circuits.len() == 1 && circuits[0].len() == self.junctions.len() {
                break;
            }
        }

        (circuits, last_connection)
    }
}

impl Solution for Day8 {
    fn part1(&self, input: &str) -> String {
        let problem = Problem::new(input);
        let (mut circuits, _) = problem.circuits(Some(1000));

        circuits.sort_by_key(|c| c.len());
        circuits
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
        let (_, (left_id, right_id)) = problem.circuits(None);

        let left_x = problem.junctions[left_id].position.x;
        let right_x = problem.junctions[right_id].position.x;

        (left_x as i64 * right_x as i64).to_string()
    }
}
