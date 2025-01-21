use std::collections::HashSet;

use itertools::Itertools;

use crate::aoc_solution::Solution;

pub struct Day12;

enum Fence {
    Vertical(Coordinate),
    Horizontal(Coordinate),
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
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

    fn get_at(&self, coordinate: Coordinate) -> &Plant {
        &self.map[coordinate.y][coordinate.x]
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn count_corners(&self, coordinate: Coordinate) -> usize {
        let mut result = 0;
        let plant = self.get_at(coordinate.clone());

        let vertical_neighbors = vec![coordinate.up(), coordinate.down(self.height())]
            .into_iter()
            .flatten()
            .filter(|coord| self.get_at(coord.clone()).kind == plant.kind)
            .count();
        let horizontal_neighbors = vec![coordinate.left(), coordinate.right(self.width())]
            .into_iter()
            .flatten()
            .filter(|coord| self.get_at(coord.clone()).kind == plant.kind)
            .count();

        if vertical_neighbors == 1 && horizontal_neighbors == 1 {
            // Inner corner
            result += 1;
        } else if (vertical_neighbors == 0 && horizontal_neighbors == 1)
            || (vertical_neighbors == 1 && horizontal_neighbors == 0)
        {
            // Line-end corner
            result += 2
        } else if vertical_neighbors == 0 && horizontal_neighbors == 0 {
            // Single area
            result += 4;
        }

        // Outer Corner
        result += vec![
            (
                coordinate.left().and_then(|it| it.up()),
                (coordinate.left(), coordinate.up()),
            ),
            (
                coordinate.right(self.width()).and_then(|it| it.up()),
                (coordinate.right(self.width()), coordinate.up()),
            ),
            (
                coordinate.left().and_then(|it| it.down(self.height())),
                (coordinate.left(), coordinate.down(self.height())),
            ),
            (
                coordinate
                    .right(self.width())
                    .and_then(|it| it.down(self.height())),
                (
                    coordinate.right(self.width()),
                    coordinate.down(self.height()),
                ),
            ),
        ]
        .into_iter()
        .flat_map(|neighbor| {
            if let Some(diag) = neighbor.0 {
                if self.get_at(diag).kind == plant.kind {
                    // Diag must be of different type
                    return None;
                }

                if let (Some(a), Some(b)) = neighbor.1 {
                    // Adjacent's must be of same type
                    if self.get_at(a).kind == plant.kind && self.get_at(b).kind == plant.kind {
                        Some(true)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .count();

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
                let number_of_sides = region
                    .iter()
                    .map(|plant| problem.count_corners(plant.position.clone()))
                    .sum::<usize>();

                region.len() * number_of_sides
            })
            .sum::<usize>()
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
            AAAAAA
            AAABBA
            AAABBA
            ABBAAA
            ABBAAA
            AAAAAA
            "
        );
        let result2 = Day12.part2(input);

        assert_eq!(result2, "368");
    }

    #[test]
    fn inner_corner() {
        let input = dedent!(
            "
            AAA
            AAA
            AAA
            "
        );
        let problem = Problem::parse_input(input);

        assert_eq!(problem.count_corners(Coordinate { x: 0, y: 0 }), 1);
        assert_eq!(problem.count_corners(Coordinate { x: 2, y: 0 }), 1);
        assert_eq!(problem.count_corners(Coordinate { x: 0, y: 2 }), 1);
        assert_eq!(problem.count_corners(Coordinate { x: 2, y: 2 }), 1);
    }

    #[test]
    fn outer_corner() {
        let input = dedent!(
            "
            AAAAAA
            AAAAAA
            AA..AA
            AAAAAA
            AAAAAA
            "
        );
        let problem = Problem::parse_input(input);

        assert_eq!(problem.count_corners(Coordinate { x: 1, y: 1 }), 1);
        assert_eq!(problem.count_corners(Coordinate { x: 4, y: 1 }), 1);
        assert_eq!(problem.count_corners(Coordinate { x: 1, y: 3 }), 1);
        assert_eq!(problem.count_corners(Coordinate { x: 4, y: 3 }), 1);
    }

    #[test]
    fn inner_and_outer_corner() {
        let input = dedent!(
            "
            AAA
            A.A
            AAA
            "
        );
        let problem = Problem::parse_input(input);

        assert_eq!(problem.count_corners(Coordinate { x: 0, y: 0 }), 2);
        assert_eq!(problem.count_corners(Coordinate { x: 2, y: 0 }), 2);
        assert_eq!(problem.count_corners(Coordinate { x: 0, y: 2 }), 2);
        assert_eq!(problem.count_corners(Coordinate { x: 2, y: 2 }), 2);
    }

    #[test]
    fn line_end_corner() {
        let input = dedent!(
            "
            AAA
            "
        );
        let problem = Problem::parse_input(input);

        assert_eq!(problem.count_corners(Coordinate { x: 0, y: 0 }), 2);
        assert_eq!(problem.count_corners(Coordinate { x: 2, y: 0 }), 2);
    }

    #[test]
    fn not_a_corner() {
        let input = dedent!(
            "
            AAA
            "
        );
        let problem = Problem::parse_input(input);

        assert_eq!(problem.count_corners(Coordinate { x: 1, y: 0 }), 0);
    }

    #[test]
    fn single_area() {
        let input = dedent!(
            "
            A
            "
        );
        let problem = Problem::parse_input(input);

        assert_eq!(problem.count_corners(Coordinate { x: 0, y: 0 }), 4);
    }
}
