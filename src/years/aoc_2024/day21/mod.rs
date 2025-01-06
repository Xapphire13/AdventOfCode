use std::{cell::RefCell, collections::HashMap, io::stdout, rc::Rc, thread::sleep, time::Duration};

use colored::Colorize;
use crossterm::{
    execute,
    terminal::{self, ClearType},
};

use crate::aoc_solution::Solution;

pub struct Day21;

const SIMULATE: bool = false;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Key {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
    Left,
    Right,
    Up,
    Down,
}

impl Key {
    fn from(char: char) -> Key {
        match char {
            '0' => Key::Zero,
            '1' => Key::One,
            '2' => Key::Two,
            '3' => Key::Three,
            '4' => Key::Four,
            '5' => Key::Five,
            '6' => Key::Six,
            '7' => Key::Seven,
            '8' => Key::Eight,
            '9' => Key::Nine,
            'A' => Key::Activate,
            '<' => Key::Left,
            '>' => Key::Right,
            '^' => Key::Up,
            'v' => Key::Down,
            _ => panic!("Invalid key {}", char),
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Key::Zero => "0",
            Key::One => "1",
            Key::Two => "2",
            Key::Three => "3",
            Key::Four => "4",
            Key::Five => "5",
            Key::Six => "6",
            Key::Seven => "7",
            Key::Eight => "8",
            Key::Nine => "9",
            Key::Activate => "A",
            Key::Left => "<",
            Key::Right => ">",
            Key::Up => "^",
            Key::Down => "v",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct XYCoordinate(u8, u8);

impl XYCoordinate {
    fn sub(&self, rhs: &XYCoordinate) -> (i8, i8) {
        (self.0 as i8 - rhs.0 as i8, self.1 as i8 - rhs.1 as i8)
    }
}

#[derive(Clone)]
struct Keypad {
    blank_space: XYCoordinate,
    buttons: HashMap<Key, XYCoordinate>,
}

impl Keypad {
    fn from(blank_space: XYCoordinate, buttons: Vec<(Key, XYCoordinate)>) -> Keypad {
        let mut button_map = HashMap::new();

        for (key, position) in buttons {
            button_map.insert(key, position);
        }

        Keypad {
            blank_space,
            buttons: button_map,
        }
    }

    fn generate_input_sequence(&self, code: Vec<Key>) -> Vec<Key> {
        let mut current_position = self.buttons.get(&Key::Activate).cloned().unwrap();
        let mut result = vec![];

        for key in code {
            let destination = self.buttons.get(&key).cloned().unwrap();
            let (dx, dy) = destination.sub(&current_position);

            let move_vertically = |result: &mut Vec<Key>| {
                for _ in 0..dy.abs() {
                    result.push(if dy < 0 { Key::Up } else { Key::Down });
                }
            };

            let move_horizontally = |result: &mut Vec<Key>| {
                for _ in 0..dx.abs() {
                    result.push(if dx < 0 { Key::Left } else { Key::Right });
                }
            };

            let need_to_move_right = dx > 0;
            let need_to_move_left = dx < 0;
            let need_to_move_up = dy < 0;
            let need_to_move_down = dy > 0;
            let horizontal_clash = current_position.1 == self.blank_space.1
                && (current_position.0 as i8 + dx) as u8 == self.blank_space.0;
            let vertical_clash = current_position.0 == self.blank_space.0
                && (current_position.1 as i8 + dy) as u8 == self.blank_space.1;

            // Order of precedence <, ^, v, >
            if need_to_move_left {
                if horizontal_clash {
                    move_vertically(&mut result);
                    move_horizontally(&mut result);
                } else {
                    move_horizontally(&mut result);
                    move_vertically(&mut result);
                }
            } else if need_to_move_up {
                if vertical_clash {
                    move_horizontally(&mut result);
                    move_vertically(&mut result);
                } else {
                    move_vertically(&mut result);
                    move_horizontally(&mut result);
                }
            } else if need_to_move_down {
                if vertical_clash {
                    move_horizontally(&mut result);
                    move_vertically(&mut result);
                } else {
                    move_vertically(&mut result);
                    move_horizontally(&mut result);
                }
            } else if need_to_move_right {
                if horizontal_clash {
                    move_vertically(&mut result);
                    move_horizontally(&mut result);
                } else {
                    move_horizontally(&mut result);
                    move_vertically(&mut result);
                }
            }

            result.push(Key::Activate);
            current_position = destination;
        }

        result
    }

    fn format(&self, hover_position: XYCoordinate, last_key_pressed: Option<Key>) -> Vec<String> {
        let mut coord_to_key = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for (key, position) in self.buttons.iter() {
            coord_to_key.insert(position.clone(), key.clone());
            max_x = max_x.max(position.0);
            max_y = max_y.max(position.1);
        }

        let mut formatted_rows = vec![];

        // Example
        //     +---+---+
        //     | ^ | A |
        // +---+---+---+
        // | < | v | > |
        // +---+---+---+

        for y in 0..=max_y {
            let mut border = String::new();
            let mut value = String::new();
            for x in 0..=max_x {
                match coord_to_key.get(&XYCoordinate(x, y)) {
                    Some(key) => {
                        let mut key_value = key.to_string().normal();

                        if hover_position == XYCoordinate(x, y) {
                            key_value = key_value.bold().yellow();

                            if last_key_pressed.clone().is_some_and(|it| it == *key) {
                                key_value = key_value.on_red();
                            }
                        }

                        border.push_str("+---");
                        value.push_str(format!("| {} ", key_value).as_str());
                    }
                    None => {
                        border.push_str(if y > 0 { "+---" } else { "    " });

                        if hover_position == XYCoordinate(x, y) {
                            let red = "  ".on_yellow();
                            value.push_str(format!(" {} ", red).as_str());
                        } else {
                            value.push_str("    ");
                        }
                    }
                }
            }
            border.push('+');
            value.push('|');

            formatted_rows.push(border);
            formatted_rows.push(value);
        }
        let mut bottom_border = String::new();
        for x in 0..=max_x {
            match coord_to_key.get(&XYCoordinate(x, max_y)) {
                Some(_) => {
                    bottom_border.push_str("+---");
                }
                None => {
                    bottom_border.push_str("    ");
                }
            }
        }
        bottom_border.push('+');
        formatted_rows.push(bottom_border);

        formatted_rows
    }
}

struct Problem {
    keypads: Vec<Keypad>,
    codes: Vec<Vec<Key>>,
}

impl Problem {
    fn parse_input(input: &str) -> Problem {
        let codes = input
            .trim()
            .lines()
            .map(|line| line.chars().map(Key::from).collect())
            .collect();

        //     +---+---+
        //     | ^ | A |
        // +---+---+---+
        // | < | v | > |
        // +---+---+---+
        let directional_keypad = Keypad::from(
            XYCoordinate(0, 0),
            vec![
                (Key::Up, XYCoordinate(1, 0)),
                (Key::Activate, XYCoordinate(2, 0)),
                (Key::Left, XYCoordinate(0, 1)),
                (Key::Down, XYCoordinate(1, 1)),
                (Key::Right, XYCoordinate(2, 1)),
            ],
        );

        // +---+---+---+
        // | 7 | 8 | 9 |
        // +---+---+---+
        // | 4 | 5 | 6 |
        // +---+---+---+
        // | 1 | 2 | 3 |
        // +---+---+---+
        //     | 0 | A |
        //     +---+---+
        let numeric_keypad = Keypad::from(
            XYCoordinate(0, 3),
            vec![
                (Key::Seven, XYCoordinate(0, 0)),
                (Key::Eight, XYCoordinate(1, 0)),
                (Key::Nine, XYCoordinate(2, 0)),
                (Key::Four, XYCoordinate(0, 1)),
                (Key::Five, XYCoordinate(1, 1)),
                (Key::Six, XYCoordinate(2, 1)),
                (Key::One, XYCoordinate(0, 2)),
                (Key::Two, XYCoordinate(1, 2)),
                (Key::Three, XYCoordinate(2, 2)),
                (Key::Zero, XYCoordinate(1, 3)),
                (Key::Activate, XYCoordinate(2, 3)),
            ],
        );

        Problem {
            keypads: vec![
                numeric_keypad,
                directional_keypad.clone(),
                directional_keypad,
            ],
            codes,
        }
    }
}

struct RobotArm {
    keypad: Keypad,
    position: XYCoordinate,
    last_key_pressed: Option<Key>,
    on_key_pressed: Box<dyn FnMut(Key)>,
}

impl RobotArm {
    fn input(&mut self, key: Key) {
        match key {
            Key::Left => {
                self.position.0 -= 1;
            }
            Key::Right => {
                self.position.0 += 1;
            }
            Key::Up => {
                self.position.1 -= 1;
            }
            Key::Down => {
                self.position.1 += 1;
            }
            Key::Activate => {
                let key = self
                    .keypad
                    .buttons
                    .iter()
                    .find_map(|(key, position)| {
                        if *position == self.position {
                            Some(key.clone())
                        } else {
                            None
                        }
                    })
                    .unwrap();

                self.last_key_pressed = Some(key.clone());
                (self.on_key_pressed)(key);
            }
            _ => {}
        }
    }
}

struct Simulation {
    robot_arms: Vec<Rc<RefCell<RobotArm>>>,
}

impl Simulation {
    fn new(problem: &Problem) -> Simulation {
        let numeric_pad = Rc::new(RefCell::new(RobotArm {
            keypad: problem.keypads[0].clone(),
            position: problem.keypads[0]
                .buttons
                .get(&Key::Activate)
                .cloned()
                .unwrap(),
            last_key_pressed: None,
            on_key_pressed: Box::new(|key| println!("Digit entered! {}", key.to_string())),
        }));

        let directional_pad_1 = Rc::new(RefCell::new(RobotArm {
            keypad: problem.keypads[1].clone(),
            position: problem.keypads[1]
                .buttons
                .get(&Key::Activate)
                .cloned()
                .unwrap(),
            last_key_pressed: None,
            on_key_pressed: Box::new(|_| {}), // Placeholder
        }));

        let directional_pad_2 = Rc::new(RefCell::new(RobotArm {
            keypad: problem.keypads[2].clone(),
            position: problem.keypads[2]
                .buttons
                .get(&Key::Activate)
                .cloned()
                .unwrap(),
            last_key_pressed: None,
            on_key_pressed: Box::new(|_| {}), // Placeholder
        }));

        let numeric_pad_ptr = numeric_pad.clone();
        let directional_pad_1_ptr = directional_pad_1.clone();

        directional_pad_1.borrow_mut().on_key_pressed = Box::new(move |key| {
            numeric_pad_ptr.borrow_mut().input(key);
        });

        directional_pad_2.borrow_mut().on_key_pressed = Box::new(move |key| {
            directional_pad_1_ptr.borrow_mut().input(key);
        });

        Simulation {
            robot_arms: vec![numeric_pad, directional_pad_1, directional_pad_2],
        }
    }

    fn input(&mut self, key: Key) {
        let robot_arm = self.robot_arms.last().unwrap();
        robot_arm.borrow_mut().input(key);
    }

    fn print(&self) {
        let mut num_rows = 0;
        let formatted_keypads = self
            .robot_arms
            .iter()
            .rev()
            .map(|robot_arm| {
                let robot_arm = robot_arm.borrow();
                let formatted = robot_arm
                    .keypad
                    .format(robot_arm.position, robot_arm.last_key_pressed.clone());

                num_rows = num_rows.max(formatted.len());

                formatted
            })
            .collect::<Vec<_>>();

        let fallback = "             ".to_string();

        for row in 0..num_rows {
            for formatted_keypad in formatted_keypads.iter() {
                let formatted_row = formatted_keypad.get(row).unwrap_or(&fallback);
                print!("{}    ", formatted_row);
            }
            println!()
        }
        println!();
    }

    fn simulate(&mut self, input_sequence: &Vec<Key>) {
        let mut io = stdout();

        execute!(io, terminal::EnterAlternateScreen).expect("Failed to enter alt screen");

        for key in input_sequence {
            execute!(io, terminal::Clear(ClearType::All)).expect("Failed to clear screen");
            self.print();
            self.input(key.clone());
            sleep(Duration::from_millis(500));
        }

        execute!(io, terminal::LeaveAlternateScreen).expect("Failed to leave alt screen");
    }
}

impl Solution for Day21 {
    fn part1(&self, input: &str) -> String {
        let problem = Problem::parse_input(input);
        let mut result = 0;

        for code in problem.codes.clone() {
            let numeric_part = code
                .iter()
                .take(3)
                .map(Key::to_string)
                .collect::<Vec<_>>()
                .join("")
                .parse::<u16>()
                .unwrap();
            let input_sequence = problem.keypads.iter().fold(code, |acc, keypad| {
                println!(
                    "{}",
                    acc.iter().map(Key::to_string).collect::<Vec<_>>().join("")
                );
                keypad.generate_input_sequence(acc)
            });

            let complexity_score = numeric_part as usize * input_sequence.len();
            result += complexity_score;

            let seq = input_sequence
                .iter()
                .map(|key| key.to_string())
                .collect::<Vec<_>>()
                .join("");
            println!("{}", seq);
            println!(
                "{} * {}, Complexity -> {}",
                input_sequence.len(),
                numeric_part,
                complexity_score
            );

            if SIMULATE {
                println!("Simulating");
                let mut simulation = Simulation::new(&problem);
                simulation.simulate(&input_sequence);
            }
        }

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        String::from("Not implemented")
    }
}

#[cfg(test)]
mod tests {
    use dedent::dedent;

    use super::*;

    #[test]
    fn test_day21() {
        let input = dedent!(
            "
            029A
            980A
            179A
            456A
            379A
            "
        );
        let result = Day21.part1(input);

        assert_eq!(result, "126384");
    }
}
