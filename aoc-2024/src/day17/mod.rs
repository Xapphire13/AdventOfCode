use regex::Regex;

use shared::Solution;

pub struct Day17;

enum Instruction {
    Adv(u8),
    Bxl(u8),
    Bst(u8),
    Jnz(u8),
    Bxc,
    Out(u8),
    Bdv(u8),
    Cdv(u8),
}

impl Instruction {
    fn from_opcode(opcode: u8, operand: u8) -> Instruction {
        match opcode {
            0 => Instruction::Adv(operand),
            1 => Instruction::Bxl(operand),
            2 => Instruction::Bst(operand),
            3 => Instruction::Jnz(operand),
            4 => Instruction::Bxc,
            5 => Instruction::Out(operand),
            6 => Instruction::Bdv(operand),
            7 => Instruction::Cdv(operand),
            _ => panic!("Invalid op code"),
        }
    }
}

struct Cpu {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_pointer: u32,
    program: Vec<u8>,
}

impl Cpu {
    fn parse_input(input: &str) -> Cpu {
        let mut lines = input.lines();
        let register_regex = Regex::new(r"Register .: (\d+)").unwrap();

        let register_a = lines
            .next()
            .map(|line| {
                let (_, [value]) = register_regex.captures(line).unwrap().extract();

                value.parse().unwrap()
            })
            .unwrap();
        let register_b = lines
            .next()
            .map(|line| {
                let (_, [value]) = register_regex.captures(line).unwrap().extract();

                value.parse().unwrap()
            })
            .unwrap();
        let register_c = lines
            .next()
            .map(|line| {
                let (_, [value]) = register_regex.captures(line).unwrap().extract();

                value.parse().unwrap()
            })
            .unwrap();

        lines.next(); // Skip empty line

        let mut program = vec![];

        let mut iterator = lines.next().unwrap().chars().skip(9);
        while let Some(value) = iterator.next() {
            program.push(value.to_digit(10).unwrap() as u8);

            iterator.next(); // Skip ','
        }

        Cpu {
            register_a,
            register_b,
            register_c,
            instruction_pointer: 0,
            program,
        }
    }

    fn execute(&mut self) -> String {
        let mut output: Vec<String> = vec![];

        while self.tick(&mut output) {}

        output.join(",")
    }

    fn tick(&mut self, output: &mut Vec<String>) -> bool {
        if self.instruction_pointer >= (self.program.len() - 1) as u32 {
            return false;
        }

        let instruction = {
            let opcode = self.program[self.instruction_pointer as usize];
            let operand = self.program[self.instruction_pointer as usize + 1];

            Instruction::from_opcode(opcode, operand)
        };

        match instruction {
            Instruction::Adv(operand) => {
                let operand = self.compute_combo_operand(operand);
                let numerator = self.register_a;
                let denominator = 2_u64.pow(operand as u32);

                self.register_a = numerator / denominator;
            }
            Instruction::Bxl(operand) => {
                self.register_b ^= operand as u64;
            }
            Instruction::Bst(operand) => {
                let operand = self.compute_combo_operand(operand);
                self.register_b = operand % 8;
            }
            Instruction::Jnz(operand) if self.register_a != 0 => {
                self.instruction_pointer = operand as u32;

                // Skip incrementing the instruction pointer
                return true;
            }
            Instruction::Bxc => {
                self.register_b ^= self.register_c;
            }
            Instruction::Out(operand) => {
                let operand = self.compute_combo_operand(operand);
                output.push((operand % 8).to_string());
            }
            Instruction::Bdv(operand) => {
                let operand = self.compute_combo_operand(operand);
                let numerator = self.register_a;
                let denominator = 2_u64.pow(operand as u32);

                self.register_b = numerator / denominator;
            }
            Instruction::Cdv(operand) => {
                let operand = self.compute_combo_operand(operand);
                let numerator = self.register_a;
                let denominator = 2_u64.pow(operand as u32);

                self.register_c = numerator / denominator;
            }
            _ => {}
        }

        self.instruction_pointer += 2;

        true
    }

    fn compute_combo_operand(&self, operand: u8) -> u64 {
        match operand {
            x if (0..=3).contains(&x) => x as u64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            invalid => panic!("Invalid combo operand {invalid}"),
        }
    }
}

impl Solution for Day17 {
    fn part1(&self, input: &str) -> String {
        let mut cpu = Cpu::parse_input(input);

        cpu.execute()
    }

    fn part2(&self, input: &str) -> String {
        let mut cpu = Cpu::parse_input(input);
        let initial_register_b = cpu.register_b;
        let initial_register_c = cpu.register_c;
        let desired_output = cpu
            .program
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let mut candidate = 258394902691840;

        loop {
            cpu.instruction_pointer = 0;
            cpu.register_a = candidate;
            cpu.register_b = initial_register_b;
            cpu.register_c = initial_register_c;

            let output = cpu.execute();

            // println!("{} <-- finding", desired_output);
            // println!("{} <--{}", output, candidate);

            if output == desired_output {
                break;
            }

            candidate += 1;
        }

        candidate.to_string()
    }
}
