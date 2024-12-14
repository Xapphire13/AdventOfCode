use std::collections::HashSet;

use crate::aoc_solution::Solution;

pub struct Day12;

enum Fence {
    Vertical(Coordinate),
    Horizontal(Coordinate),
}

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

    fn region_perimeter(&self, region: &Vec<&Plant>) -> Vec<Fence> {
        let mut result = vec![];
        let map_height = self.map.len();
        let map_width = self.map[0].len();

        for plant in region {
            if let Some(Coordinate { x, y }) = plant.position.left() {
                let neighbor = &self.map[y][x];

                // Perimeter along plant kind boundaries
                if neighbor.kind != plant.kind {
                    result.push(Fence::Vertical(Coordinate {
                        x: plant.position.x,
                        y: plant.position.y,
                    }));
                }
            } else {
                // Map boundary counts as perimeter
                result.push(Fence::Vertical(Coordinate {
                    x: plant.position.x,
                    y: plant.position.y,
                }));
            }

            if let Some(Coordinate { x, y }) = plant.position.right(map_width) {
                let neighbor = &self.map[y][x];

                // Perimeter along plant kind boundaries
                if neighbor.kind != plant.kind {
                    result.push(Fence::Vertical(Coordinate {
                        x: plant.position.x + 1,
                        y: plant.position.y,
                    }));
                }
            } else {
                // Map boundary counts as perimeter
                result.push(Fence::Vertical(Coordinate {
                    x: plant.position.x + 1,
                    y: plant.position.y,
                }));
            }

            if let Some(Coordinate { x, y }) = plant.position.up() {
                let neighbor = &self.map[y][x];

                // Perimeter along plant kind boundaries
                if neighbor.kind != plant.kind {
                    result.push(Fence::Horizontal(Coordinate {
                        x: plant.position.x,
                        y: plant.position.y,
                    }));
                }
            } else {
                // Map boundary counts as perimeter
                result.push(Fence::Horizontal(Coordinate {
                    x: plant.position.x,
                    y: plant.position.y,
                }));
            }

            if let Some(Coordinate { x, y }) = plant.position.down(map_height) {
                let neighbor = &self.map[y][x];

                // Perimeter along plant kind boundaries
                if neighbor.kind != plant.kind {
                    result.push(Fence::Horizontal(Coordinate {
                        x: plant.position.x,
                        y: plant.position.y + 1,
                    }));
                }
            } else {
                // Map boundary counts as perimeter
                result.push(Fence::Horizontal(Coordinate {
                    x: plant.position.x,
                    y: plant.position.y + 1,
                }));
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
            .map(|region| region.len() as u32 * problem.region_perimeter(region).len() as u32)
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let problem = Problem::parse_input(input);

        problem
            .find_regions()
            .iter()
            .map(|region| {
                let fences = problem.region_perimeter(region);

                let mut vertical_positions = fences
                    .iter()
                    .filter_map(|fence| match fence {
                        Fence::Vertical(position) => Some(position),
                        Fence::Horizontal(_) => None,
                    })
                    .collect::<Vec<_>>();
                vertical_positions.sort_by(|a, b| match a.x.cmp(&b.x) {
                    std::cmp::Ordering::Equal => a.y.cmp(&b.y),
                    it => it,
                });
                let mut vertical_sections = if vertical_positions.len() > 0 { 1 } else { 0 };
                for (i, &section) in vertical_positions.iter().enumerate().skip(1) {
                    let prev_section = vertical_positions[i - 1];

                    if prev_section.x != section.x || prev_section.y != section.y - 1 {
                        vertical_sections += 1;
                    }
                }

                let mut horizontal_positions = fences
                    .iter()
                    .filter_map(|fence| match fence {
                        Fence::Vertical(_) => None,
                        Fence::Horizontal(position) => Some(position),
                    })
                    .collect::<Vec<_>>();
                horizontal_positions.sort_by(|a, b| match a.y.cmp(&b.y) {
                    std::cmp::Ordering::Equal => a.x.cmp(&b.x),
                    it => it,
                });
                let mut horizontal_sections = if horizontal_positions.len() > 0 { 1 } else { 0 };
                for (i, &section) in horizontal_positions.iter().enumerate().skip(1) {
                    let prev_section = horizontal_positions[i - 1];

                    if prev_section.y != section.y || prev_section.x != section.x - 1 {
                        horizontal_sections += 1;
                    }
                }

                region.len() as u32 * (vertical_sections + horizontal_sections)
            })
            .sum::<u32>()
            .to_string()
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
        let result1 = Day12.part1(input);
        let result2 = Day12.part2(input);

        assert_eq!(result1, "1930");
        assert_eq!(result2, "1206");
    }

    #[test]
    fn test_day12_2() {
        let input = dedent!(
            "
            EEEEE
            EXXXX
            EEEEE
            EXXXX
            EEEEE
            "
        );
        let result2 = Day12.part2(input);

        assert_eq!(result2, "236");
    }
}
