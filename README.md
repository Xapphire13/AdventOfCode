# Advent of Code

Solutions for Advent of Code challenges organized as a Rust workspace.

## Structure

```
advent_of_code/
├── Cargo.toml          # Workspace configuration
├── shared/             # Shared utilities (Solution trait, helpers)
├── aoc-2022/           # 2022 solutions (library crate)
├── aoc-2024/           # 2024 solutions (library crate)
└── runner/             # CLI runner (binary crate)
```

## Usage

### Run a solution

```bash
cargo run --bin aoc -- run <year> <day>

# Examples:
cargo run --bin aoc -- run 2024 1
cargo run --bin aoc -- run 2022 5
```

### Create a new day template

```bash
cargo run --bin aoc -- create <year> <day>

# Example:
cargo run --bin aoc -- create 2024 10
```

This will:
- Create `aoc-<year>/src/day<XX>/mod.rs` with Solution trait template
- Create `aoc-<year>/src/day<XX>/input.txt` for puzzle input
- Remind you to update the year's `lib.rs`

### Add a new year

1. Create a new library crate:
   ```bash
   cargo new --lib aoc-<year>
   ```

2. Add it to the workspace in root `Cargo.toml`:
   ```toml
   members = ["shared", "aoc-2022", "aoc-2024", "aoc-<year>", "runner"]
   ```

3. Add dependencies to `aoc-<year>/Cargo.toml`:
   ```toml
   [dependencies]
   shared = { path = "../shared" }
   colored = { workspace = true }
   itertools = { workspace = true }
   # ... other dependencies
   ```

4. Update `runner/src/main.rs` to include the new year in the match statement

## Development

### Build the workspace

```bash
cargo build --workspace
```

### Build a specific crate

```bash
cargo build -p aoc-2024
cargo build -p runner
```

### Run tests

```bash
cargo test --workspace
```
