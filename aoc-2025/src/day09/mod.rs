use shared::{Coordinate, Solution};

pub struct Day9;

impl Solution for Day9 {
    fn part1(&self, input: &str) -> String {
        let mut grid_size = 0;
        let red_tile_locations = input
            .trim()
            .lines()
            .map(|l| {
                let (col_str, row_str) = l.split_once(',').unwrap();
                let row = row_str.parse().unwrap();
                let col = col_str.parse().unwrap();
                grid_size = grid_size.max(row).max(col);
                Coordinate::new(row, col)
            })
            .collect::<Vec<_>>();

        let mut largest_area = 0;
        for i in 0..red_tile_locations.len() {
            let first = &red_tile_locations[i];
            for j in (i + 1)..red_tile_locations.len() {
                let second = &red_tile_locations[j];

                largest_area = largest_area.max(first.area_between(second));
            }
        }

        largest_area.to_string()
    }

    fn part2(&self, input: &str) -> String {
        String::from("todo")
    }
}
