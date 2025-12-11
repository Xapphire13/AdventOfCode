use shared::Solution;

pub struct Day3;

impl Solution for Day3 {
    fn part1(&self, input: &str) -> String {
        let mut result = 0u32;
        let banks = parse_input(input);

        for bank in banks {
            // Find largest leading number
            let leading_digit = bank[..bank.len() - 1]
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|pair| pair.1)
                .unwrap();

            let trailing_digit = *bank[(leading_digit.0 + 1)..].iter().max().unwrap();

            let joltage: u32 = format!("{}{trailing_digit}", leading_digit.1)
                .parse()
                .unwrap();

            result += joltage;
        }

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut result = 0u64;
        let banks = parse_input(input);

        for bank in banks {
            let mut digits_remaining = 12;
            let mut batteries = vec![];
            let mut start_index = 0;

            while digits_remaining > 0 {
                let (index, digit) = pick_digit(&bank[start_index..], digits_remaining - 1);
                batteries.push(digit);
                start_index += index + 1;

                digits_remaining -= 1;
            }

            let joltage: u64 = batteries
                .iter()
                .map(|battery| battery.to_string())
                .collect::<Vec<_>>()
                .join("")
                .parse()
                .unwrap();

            result += joltage;
        }

        result.to_string()
    }
}

fn pick_digit(bank: &[u8], reserved_space: usize) -> (usize, u8) {
    let (index, value) = bank[..bank.len() - reserved_space]
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|pair| pair.1)
        .unwrap();

    (index, *value)
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}
