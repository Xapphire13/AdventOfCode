use std::collections::HashSet;

use crate::aoc_solution::Solution;

pub struct Day12;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn left(&self) -> Option<Coordinate> {
        let Coordinate { x, y } = *self;

        if x == 0 {
            return None;
        }

        Some(Coordinate { x: x - 1, y })
    }
    fn right(&self, bounds: usize) -> Option<Coordinate> {
        let Coordinate { x, y } = *self;

        if x >= bounds - 1 {
            return None;
        }

        Some(Coordinate { x: x + 1, y })
    }
    fn up(&self) -> Option<Coordinate> {
        let Coordinate { x, y } = *self;

        if y == 0 {
            return None;
        }

        Some(Coordinate { x, y: y - 1 })
    }
    fn down(&self, bounds: usize) -> Option<Coordinate> {
        let Coordinate { x, y } = *self;

        if y >= bounds - 1 {
            return None;
        }

        Some(Coordinate { x, y: y + 1 })
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Plant {
    kind: char,
    position: Coordinate,
}

struct Problem {
    map: Vec<Vec<Plant>>,
}

impl Problem {
    fn parse_input(input: &str) -> Problem {
        let map = input
            .trim()
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, kind)| Plant {
                        kind,
                        position: Coordinate { x: col, y: row },
                    })
                    .collect()
            })
            .collect();

        Problem { map }
    }

    fn find_regions(&self) -> Vec<Vec<&Plant>> {
        let mut plants_with_regions = HashSet::new();
        let mut regions = vec![];

        fn dfs<'a>(
            start_plant: &Plant,
            region: &mut Vec<&'a Plant>,
            plants_with_regions: &mut HashSet<&'a Plant>,
            map: &'a Vec<Vec<Plant>>,
        ) {
            let map_height = map.len();
            let map_width = map[0].len();

            let plants = [
                start_plant.position.left(),
                start_plant.position.right(map_width),
                start_plant.position.up(),
                start_plant.position.down(map_height),
            ]
            .iter()
            .flatten()
            .map(|Coordinate { x, y }| &map[*y][*x])
            .filter(|plant| plant.kind == start_plant.kind && !plants_with_regions.contains(plant))
            .collect::<Vec<_>>();

            for plant in plants.iter() {
                region.push(plant);
                plants_with_regions.insert(plant);
            }

            for plant in plants {
                dfs(plant, region, plants_with_regions, map);
            }
        }

        for plant in self.map.iter().flatten() {
            if plants_with_regions.contains(plant) {
                // Already placed into a region
                continue;
            }

            // This plant hasn't been placed yet, start a new region for it
            let mut region = vec![];
            region.push(plant);
            plants_with_regions.insert(plant);

            // Find other plants in the region
            dfs(plant, &mut region, &mut plants_with_regions, &self.map);

            regions.push(region);
        }

        regions
    }

    fn region_perimeter(&self, region: &Vec<&Plant>) -> u32 {
        let mut result = 0;
        let map_height = self.map.len();
        let map_width = self.map[0].len();

        for plant in region {
            for position in [
                plant.position.left(),
                plant.position.right(map_width),
                plant.position.up(),
                plant.position.down(map_height),
            ] {
                if let Some(Coordinate { x, y }) = position {
                    let neighbor = &self.map[y][x];

                    // Perimeter along plant kind boundaries
                    if neighbor.kind != plant.kind {
                        result += 1;
                    }
                } else {
                    // Map boundary counts as perimeter
                    result += 1;
                }
            }
        }

        result
    }
}

impl Solution for Day12 {
    fn part1(&self, input: &str) -> String {
        let problem = Problem::parse_input(input);

        problem
            .find_regions()
            .iter()
            .map(|region| region.len() as u32 * problem.region_perimeter(region))
            .sum::<u32>()
            .to_string()
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
    fn test_day12() {
        let input = dedent!(
            "
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
            "
        );
        let result = Day12.part1(input);

        assert_eq!(result, "1930");
    }
}
