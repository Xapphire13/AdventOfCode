use core::panic;
use std::collections::VecDeque;

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2:");
    part2(&input);
}

#[derive(Clone, Copy, PartialEq)]
enum Instruction {
    Noop,
    AddX(i32),
}

struct Cpu {
    cycle: usize,
    register: i32,
    /** The currently executing instruction */
    instruction: Option<Instruction>,
    program: VecDeque<Instruction>,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            cycle: 0,
            register: 1,
            instruction: None,
            program: VecDeque::new(),
        }
    }

    fn tick(&mut self) {
        // Start cycle
        match self.instruction {
            Some(inst) => match inst {
                Instruction::AddX(arg) => {
                    self.register += arg;
                    self.instruction = None;
                }
                _ => panic!("Can't execute instruction"),
            },
            None => {
                let inst = self.program.pop_front();
                self.instruction = inst;
            }
        }

        // End cycle
        self.cycle += 1;

        // Remove Noop at the end of the cycle
        if let Some(inst) = self.instruction {
            if inst == Instruction::Noop {
                self.instruction = None;
            }
        }
    }

    fn load(&mut self, program: &[Instruction]) {
        self.program = VecDeque::from_iter(program.iter().copied())
    }

    fn is_done(&self) -> bool {
        self.instruction.is_none() && self.program.is_empty()
    }

    fn signal(&self) -> i32 {
        (self.cycle as i32) * self.register
    }
}

fn parse_input(input: &[String]) -> Vec<Instruction> {
    input
        .iter()
        .map(|line| {
            let mut split = line.split(' ');
            let inst = split.next().unwrap();
            let arg = split.next();

            match inst {
                "noop" => Instruction::Noop,
                "addx" => Instruction::AddX(arg.unwrap().parse().unwrap()),
                _ => panic!("Unsupported instruction"),
            }
        })
        .collect()
}

struct Crt {
    /** Buffer for 6x40 (Height x Width) display */
    buffer: [[char; 40]; 6],
}

impl Crt {
    fn new() -> Crt {
        Crt {
            buffer: [[' '; 40]; 6],
        }
    }

    fn draw(&mut self, cpu: &Cpu) {
        let sprite_position = cpu.register;
        let pixel = cpu.cycle;
        let row = pixel / 40;
        let col = pixel % 40;

        self.buffer[row][col] = if ((col as i32) - 1..=(col as i32) + 1).contains(&sprite_position)
        {
            '█'
        } else {
            '░'
        };
    }

    fn display(&self) {
        for row in self.buffer {
            for col in row {
                print!("{}", col);
            }

            println!();
        }
    }
}

fn part1(input: &[String]) -> i32 {
    let program = parse_input(input);
    let mut cpu = Cpu::new();
    cpu.load(&program);
    let mut result = 0;

    while !cpu.is_done() {
        cpu.tick();

        if [20, 60, 100, 140, 180, 220].contains(&cpu.cycle) {
            result += cpu.signal()
        }
    }

    result
}

fn part2(input: &[String]) {
    let program = parse_input(input);
    let mut cpu = Cpu::new();
    let mut crt = Crt::new();
    cpu.load(&program);

    while !cpu.is_done() {
        crt.draw(&cpu);
        cpu.tick();
    }

    crt.display();
}
