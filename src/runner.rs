use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn create_year_structure(year: u32) -> io::Result<()> {
    let year_folder = format!("src/years/aoc_{}", year);
    fs::create_dir_all(&year_folder)?;

    // Create years/mod.rs if it doesn't exist
    let years_mod_path = "src/years/mod.rs";
    if !Path::new(years_mod_path).exists() {
        let mut years_mod_file = fs::File::create(years_mod_path)?;
        years_mod_file.write_all(b"// Advent of Code years module\n}")?;
    }

    // Update years/mod.rs to include the new year
    let years_mod_content = fs::read_to_string(years_mod_path)?;
    if !years_mod_content.contains(&format!("pub mod aoc_{};", year)) {
        let mut updated_content = years_mod_content.trim().to_string();
        updated_content.push_str(&format!("\npub mod aoc_{};", year));

        fs::write(years_mod_path, updated_content)?;
    }

    Ok(())
}

fn create_day_solution(year: u32, day: u32) -> io::Result<()> {
    let day_folder = format!("src/years/aoc_{}/day{:02}", year, day);
    let mod_path = format!("{}/mod.rs", day_folder);
    let input_path = format!("{}/input.txt", day_folder);

    // Create day folder if it doesn't exist
    fs::create_dir_all(&day_folder)?;

    // Ensure year structure exists
    create_year_structure(year)?;

    // Create mod.rs file if it doesn't exist
    if !Path::new(&mod_path).exists() {
        let mut mod_file = fs::File::create(&mod_path)?;
        mod_file.write_all(
            format!(
                "use crate::aoc_solution::Solution;

pub struct Day{day};

impl Solution for Day{day} {{
    fn part1(&self, input: &str) -> String {{
        // Implement Part 1 solution
        String::from(\"Not implemented\")
    }}

    fn part2(&self, input: &str) -> String {{
        // Implement Part 2 solution
        String::from(\"Not implemented\")
    }}
}}"
            )
            .as_bytes(),
        )?;
    }

    // Create empty input.txt if it doesn't exist
    if !Path::new(&input_path).exists() {
        fs::File::create(&input_path)?;
    }

    // Update the year's mod.rs to include the new day
    let year_mod_path = format!("src/years/aoc_{}/mod.rs", year);

    // Create the year's mod.rs if it doesn't exist
    if !Path::new(&year_mod_path).exists() {
        let mut year_mod_file = fs::File::create(&year_mod_path)?;
        year_mod_file.write_all(b"// Advent of Code year module\n}")?;
    }

    let year_mod_content = fs::read_to_string(&year_mod_path)?;
    if !year_mod_content.contains(&format!("pub mod day{:02};", day)) {
        let mut updated_content = year_mod_content;
        updated_content.push_str(&format!("\npub mod day{:02};", day));

        fs::write(&year_mod_path, updated_content)?;
    }

    // Add to day runner
    let run_day_path = "src/run_day.rs";
    let run_day_content = fs::read_to_string(run_day_path)?;
    if let Some(insertion_index) = run_day_content
        .lines()
        .position(|line| line.contains("// Add more year and day solutions here"))
    {
        let new_line = format!(
            "        ({0}, {1}) => run_day_solution(years::aoc_2024::day{1:02}::Day{1}, input),",
            year, day
        );

        if !run_day_content.contains(&new_line) {
            let mut new_lines: Vec<&str> = run_day_content.lines().collect();
            new_lines.insert(insertion_index, new_line.as_str());

            fs::write(&run_day_path, new_lines.join("\n"))?;
        }
    }

    println!("Created solution files for Year {} Day {}", year, day);
    Ok(())
}

fn run_day_solution(year: u32, day: u32) -> io::Result<()> {
    // Dynamically load the day's solution
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "run_day",
            "--",
            &year.to_string(),
            &day.to_string(),
        ])
        .output()?;

    if output.status.success() {
        println!("Year {} Day {} Solution Output:", year, day);
        print!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Error running Year {} Day {} solution:", year, day);
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  cargo run --bin runner create <year> <day>   - Create solution files for a specific year and day");
        println!("  cargo run --bin runner run <year> <day>      - Run solution for a specific year and day");
        return Ok(());
    }

    match args[1].as_str() {
        "create" => {
            if args.len() != 4 {
                eprintln!("Please provide a year and day number to create");
                return Ok(());
            }
            let year: u32 = match args[2].parse() {
                Ok(n) if n >= 2015 && n <= 2099 => n,
                _ => {
                    eprintln!("Invalid year. Year must be between 2015 and 2099.");
                    return Ok(());
                }
            };
            let day: u32 = match args[3].parse() {
                Ok(n) if n > 0 && n <= 25 => n,
                _ => {
                    eprintln!("Invalid day. Day must be between 1 and 25.");
                    return Ok(());
                }
            };
            create_day_solution(year, day)
        }
        "run" => {
            if args.len() != 4 {
                eprintln!("Please provide a year and day number to run");
                return Ok(());
            }
            let year: u32 = match args[2].parse() {
                Ok(n) if n >= 2015 && n <= 2099 => n,
                _ => {
                    eprintln!("Invalid year. Year must be between 2015 and 2099.");
                    return Ok(());
                }
            };
            let day: u32 = match args[3].parse() {
                Ok(n) if n > 0 && n <= 25 => n,
                _ => {
                    eprintln!("Invalid day. Day must be between 1 and 25.");
                    return Ok(());
                }
            };
            run_day_solution(year, day)
        }
        _ => {
            eprintln!("Invalid command. Use 'create' or 'run'.");
            Ok(())
        }
    }
}
