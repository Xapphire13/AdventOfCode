use colored::Colorize;
use quote::quote;
use shared::Solution;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::process;
use std::time::Instant;
use syn::{parse_file, File, Item, ItemMod, Visibility};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "run" => {
            if args.len() != 4 {
                eprintln!("Usage: aoc run <year> <day>");
                process::exit(1);
            }

            let year: u32 = parse_year(&args[2]);
            let day: u32 = parse_day(&args[3]);

            run_solution(year, day);
        }
        "create" => {
            if args.len() != 4 {
                eprintln!("Usage: aoc create <year> <day>");
                process::exit(1);
            }

            let year: u32 = parse_year(&args[2]);
            let day: u32 = parse_day(&args[3]);

            create_solution(year, day);
        }
        _ => {
            eprintln!("Unknown command: {command}");
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Advent of Code Runner");
    println!();
    println!("Usage:");
    println!("  aoc run <year> <day>     - Run solution for a specific year and day");
    println!("  aoc create <year> <day>  - Create solution template for a specific year and day");
    println!();
    println!("Examples:");
    println!("  aoc run 2024 1");
    println!("  aoc create 2024 2");
}

fn parse_year(year_str: &str) -> u32 {
    match year_str.parse() {
        Ok(n) if (2015..=2099).contains(&n) => n,
        _ => {
            eprintln!("Invalid year. Must be between 2015 and 2099.");
            process::exit(1);
        }
    }
}

fn parse_day(day_str: &str) -> u32 {
    match day_str.parse() {
        Ok(n) if (1..=25).contains(&n) => n,
        _ => {
            eprintln!("Invalid day. Must be between 1 and 25.");
            process::exit(1);
        }
    }
}

fn run_solution(year: u32, day: u32) {
    let input_path = format!("aoc-{year}/src/day{day:02}/input.txt");

    println!(
        "{}",
        format!("=== Advent of Code {year} - Day {day} ===")
            .bright_cyan()
            .bold()
    );
    println!();

    match year {
        2022 => aoc_2022::run_day(day, &input_path),
        2024 => aoc_2024::run_day(day, &input_path),
        2025 => {
            if let Some(solution) = aoc_2025::get_solutions().into_iter().find_map(|solution| {
                if solution.0 == day {
                    Some(solution.1)
                } else {
                    None
                }
            }) {
                run_day(solution, &input_path)
            } else {
                eprintln!("Day {day} not implemented for 2025")
            }
        }
        _ => {
            eprintln!("Year {year} not implemented");
            process::exit(1);
        }
    }
}

/// Runs the solution for a specific day
pub fn run_day(solution: Box<dyn Solution>, input_path: &str) {
    let input = fs::read_to_string(input_path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {input_path}"));

    let start = Instant::now();
    let result = solution.part1(&input);
    let duration = start.elapsed();
    println!("Part 1: {result} ({duration:?})");

    let start = Instant::now();
    let result = solution.part2(&input);
    let duration = start.elapsed();
    println!("Part 2: {result} ({duration:?})");
}

fn create_solution(year: u32, day: u32) {
    use std::fs;
    use std::path::Path;

    let crate_name = format!("aoc-{year}");
    let day_folder = format!("{crate_name}/src/day{day:02}");
    let mod_path = format!("{day_folder}/mod.rs");
    let input_path = format!("{day_folder}/input.txt");

    // Check if crate exists
    if !Path::new(&crate_name).exists() {
        eprintln!("Error: Year {year} crate doesn't exist. Create the crate first.");
        process::exit(1);
    }

    // Create day folder
    if let Err(e) = fs::create_dir_all(&day_folder) {
        eprintln!("Error creating directory {day_folder}: {e}");
        process::exit(1);
    }

    // Create mod.rs if it doesn't exist
    if !Path::new(&mod_path).exists() {
        let template = format!(
            "use shared::Solution;\n\npub struct Day{day};\n\nimpl Solution for Day{day} {{\n    fn part1(&self, input: &str) -> String {{\n        String::from(\"todo\")\n    }}\n\n    fn part2(&self, input: &str) -> String {{\n        String::from(\"todo\")\n    }}\n}}"
        );

        if let Err(e) = fs::write(&mod_path, template) {
            eprintln!("Error writing {mod_path}: {e}");
            process::exit(1);
        }
        println!("{}", format!("✓ Created {mod_path}").green());
    } else {
        println!("{}", format!("✓ {mod_path} already exists").yellow());
    }

    // Create input.txt if it doesn't exist
    if !Path::new(&input_path).exists() {
        if let Err(e) = fs::write(&input_path, "") {
            eprintln!("Error writing {input_path}: {e}");
            process::exit(1);
        }
        println!("{}", format!("✓ Created {input_path}").green());
    } else {
        println!("{}", format!("✓ {input_path} already exists").yellow());
    }

    // Update lib.rs to include the new day module
    let lib_path = format!("{crate_name}/src/lib.rs");
    match update_lib_rs(&lib_path, day) {
        Ok(updated) => {
            if updated {
                println!("{}", format!("✓ Updated {lib_path}").green());

                // Run cargo fmt on the updated file
                if let Err(e) = std::process::Command::new("cargo")
                    .args(["fmt", "--", &lib_path])
                    .output()
                {
                    eprintln!("Warning: Failed to run cargo fmt: {e}");
                }
            } else {
                println!("{}", format!("✓ Day {day} already in {lib_path}").yellow());
            }
        }
        Err(e) => {
            eprintln!("Error updating {lib_path}: {e}");
            println!(
                "{}",
                format!("⚠ Don't forget to add 'pub mod day{day:02};' to {crate_name}/src/lib.rs")
                    .yellow()
            );
            println!(
                "{}",
                format!("⚠ And add day {day} to the get_solutions() function").yellow()
            );
        }
    }

    println!();
    println!(
        "{}",
        format!("Solution template created for Year {year} Day {day}")
            .bright_green()
            .bold()
    );
}

fn update_lib_rs(lib_path: &str, day_num: u32) -> Result<bool, Box<dyn std::error::Error>> {
    let source_code = fs::read_to_string(lib_path)?;
    let mut ast: File = parse_file(&source_code)?;

    // Collect all existing day modules
    let mut days = BTreeSet::new();
    for item in &ast.items {
        if let Item::Mod(ItemMod {
            vis: Visibility::Public(_),
            ident,
            semi: Some(_),
            ..
        }) = item
        {
            let ident_str = ident.to_string();
            if let Some(stripped) = ident_str.strip_prefix("day") {
                if let Ok(num) = stripped.parse::<u32>() {
                    days.insert(num);
                }
            }
        }
    }

    // Check if day already exists
    if days.contains(&day_num) {
        return Ok(false);
    }

    // Add the new day
    days.insert(day_num);

    // Rebuild the AST items
    let mut new_items = Vec::new();

    // Add use statement
    new_items.push(syn::parse_quote! {
        use shared::Solution;
    });

    // Add all module declarations
    for day in &days {
        let day_str = syn::Ident::new(&format!("day{:02}", day), proc_macro2::Span::call_site());
        new_items.push(syn::parse_quote! {
            pub mod #day_str;
        });
    }

    // Build the vec entries
    let vec_entries: Vec<_> = days
        .iter()
        .map(|day| {
            let day_mod =
                syn::Ident::new(&format!("day{:02}", day), proc_macro2::Span::call_site());
            let day_struct =
                syn::Ident::new(&format!("Day{}", day), proc_macro2::Span::call_site());
            quote! {
                (#day, Box::new(#day_mod::#day_struct))
            }
        })
        .collect();

    // Add the get_solutions function
    new_items.push(syn::parse_quote! {
        pub fn get_solutions() -> Vec<(u32, Box<dyn Solution>)> {
            vec![#(#vec_entries),*]
        }
    });

    ast.items = new_items;

    // Format with prettyplease
    let formatted = prettyplease::unparse(&ast);

    // Add blank line before get_solutions function
    let mut result = String::new();
    for line in formatted.lines() {
        if line.trim().starts_with("pub fn get_solutions") {
            result.push('\n');
        }
        result.push_str(line);
        result.push('\n');
    }

    fs::write(lib_path, result)?;

    Ok(true)
}
