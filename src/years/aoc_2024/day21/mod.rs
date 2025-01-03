use std::collections::HashMap;

use crate::aoc_solution::Solution;

pub struct Day21;

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

            // If moving vertically first would cause entering the blank space,
            // then move horizontally first
            if current_position.0 == self.blank_space.0
                && (current_position.1 as i8 + dy) as u8 == self.blank_space.1
            {
                move_horizontally(&mut result);
                move_vertically(&mut result);
            } else {
                move_vertically(&mut result);
                move_horizontally(&mut result);
            }

            result.push(Key::Activate);
            current_position = destination;
        }

        result
    }
}

struct Problem {
    keypads: Vec<Keypad>,
    codes: Vec<Vec<Key>>,
}

fn code_to_string(code: &Vec<Key>) -> String {
    code.iter().map(Key::to_string).collect::<Vec<_>>().join("")
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
            let input_sequence = problem
                .keypads
                .iter()
                .fold(code, |acc, keypad| keypad.generate_input_sequence(acc));

            let complexity_score = numeric_part as usize * input_sequence.len();
            result += complexity_score;

            println!(
                "{} * {}, Complexity -> {}",
                input_sequence.len(),
                numeric_part,
                complexity_score
            );
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
