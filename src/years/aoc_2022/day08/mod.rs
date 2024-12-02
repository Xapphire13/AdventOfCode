pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

/** Grid of trees, indexed by `row` then `col` */
type TreeGrid = Vec<Vec<u8>>;

/** A tree is visible if there are no trees higher than it between it and any edge */
fn is_visible(grid: &TreeGrid, row: usize, col: usize) -> bool {
    if col == 0 || col == grid[0].len() - 1 || row == 0 || row == grid.len() - 1 {
        // Edge tree
        return true;
    }

    let height = grid[row][col];
    let mut visible = true;

    // Left Edge
    for c in 0..col {
        if grid[row][c] >= height {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    visible = true;
    // Right Edge
    for c in (col + 1)..grid[0].len() {
        if grid[row][c] >= height {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    visible = true;

    // Top Edge
    for r in 0..row {
        if grid[r][col] >= height {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    visible = true;

    // Bottom Edge
    for r in (row + 1)..grid.len() {
        if grid[r][col] >= height {
            visible = false;
            break;
        }
    }

    visible
}

/** A tree's scenic score is the number of tree's it can see in each direction, multiplied together */
fn calculate_scenic_score(grid: &TreeGrid, row: usize, col: usize) -> usize {
    let height = grid[row][col];
    let mut left = 0;
    let mut right = 0;
    let mut up = 0;
    let mut down = 0;

    // Left
    for c in (0..col).rev() {
        if grid[row][c] <= height {
            left += 1
        }

        if grid[row][c] >= height {
            break;
        }
    }

    // Right
    for c in (col + 1)..grid[0].len() {
        if grid[row][c] <= height {
            right += 1;
        }

        if grid[row][c] >= height {
            break;
        }
    }

    // Up
    for r in (0..row).rev() {
        if grid[r][col] <= height {
            up += 1;
        }

        if grid[r][col] >= height {
            break;
        }
    }

    // Down
    for r in (row + 1)..grid.len() {
        if grid[r][col] <= height {
            down += 1;
        }

        if grid[r][col] >= height {
            break;
        }
    }

    left * right * up * down
}

fn parse_input(input: &[String]) -> TreeGrid {
    let mut grid = vec![];

    for line in input {
        let mut row = vec![];
        for tree_height in line.chars() {
            row.push(tree_height.to_digit(10).unwrap() as u8);
        }

        grid.push(row);
    }

    grid
}

fn part1(input: &[String]) -> usize {
    let grid = parse_input(input);
    let mut result = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if is_visible(&grid, row, col) {
                result += 1;
            }
        }
    }

    result
}

fn part2(input: &[String]) -> usize {
    let grid = parse_input(input);
    let mut result = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {
            result = result.max(calculate_scenic_score(&grid, row, col));
        }
    }

    result
}
