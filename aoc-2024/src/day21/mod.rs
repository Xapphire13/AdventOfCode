use std::collections::HashMap;

use itertools::Itertools;

use shared::Solution;

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
            '<' => Key::Left,
            '>' => Key::Right,
            '^' => Key::Up,
            'v' => Key::Down,
            _ => panic!("Invalid key {char}"),
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
    /** (from, to) -> possible input sequences for move */
    input_sequences: HashMap<(Key, Key), Vec<Vec<Key>>>,
}

impl Keypad {
    fn from(blank_space: XYCoordinate, buttons: Vec<(Key, XYCoordinate)>) -> Keypad {
        let mut input_sequences = HashMap::new();

        for (from_key, from_pos) in buttons.iter() {
            for (to_key, to_pos) in buttons.iter() {
                let mut sequences = vec![];
                let (dx, dy) = to_pos.sub(from_pos);

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

                let horizontal_clash =
                    from_pos.1 == blank_space.1 && (from_pos.0 as i8 + dx) as u8 == blank_space.0;
                let vertical_clash =
                    from_pos.0 == blank_space.0 && (from_pos.1 as i8 + dy) as u8 == blank_space.1;

                if !horizontal_clash {
                    let mut horizontal_first_sequence = vec![];
                    move_horizontally(&mut horizontal_first_sequence);
                    move_vertically(&mut horizontal_first_sequence);
                    sequences.push(horizontal_first_sequence);
                }

                if !vertical_clash {
                    let mut vertical_first_sequence = vec![];
                    move_vertically(&mut vertical_first_sequence);
                    move_horizontally(&mut vertical_first_sequence);
                    sequences.push(vertical_first_sequence);
                }

                input_sequences.insert((from_key.clone(), to_key.clone()), sequences);
            }
        }

        Keypad { input_sequences }
    }
}

struct Problem {
    keypads: Vec<Keypad>,
    codes: Vec<Vec<Key>>,
}

impl Problem {
    fn parse_input(input: &str, number_of_directional_pads: u8) -> Problem {
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

        let mut keypads = vec![numeric_keypad];

        for _ in 0..number_of_directional_pads {
            keypads.push(directional_keypad.clone());
        }

        Problem { keypads, codes }
    }

    fn get_input_sequence(
        &self,
        memo: &mut HashMap<usize, HashMap<String, usize>>,
        depth: usize,
        result_sequence: Vec<Key>,
    ) -> usize {
        let result_sequence_string = result_sequence.iter().map(|key| key.to_string()).join("");

        if depth == self.keypads.len() {
            return result_sequence_string.len();
        }

        if let Some(sequence_length) = memo
            .get(&depth)
            .and_then(|depth_memo| depth_memo.get(&result_sequence_string))
        {
            return *sequence_length;
        }

        let mut result = 0;

        let keypad = &self.keypads[depth];
        let mut from = &Key::Activate;
        for to in result_sequence.iter() {
            let sub_sequences = keypad
                .input_sequences
                .get(&(from.clone(), to.clone()))
                .unwrap_or_else(|| {
                    panic!(
                        "from: {}, to: {}, depth: {}\n{:#?}",
                        from.to_string(),
                        to.to_string(),
                        depth,
                        keypad.input_sequences
                    )
                });

            result += sub_sequences
                .iter()
                .cloned()
                .map(|mut sub_sequence| {
                    sub_sequence.push(Key::Activate);
                    self.get_input_sequence(memo, depth + 1, sub_sequence)
                })
                .min()
                .unwrap();

            from = to;
        }

        memo.entry(depth)
            .and_modify(|depth_memo| {
                depth_memo.insert(result_sequence_string.clone(), result);
            })
            .or_insert_with(|| HashMap::from([(result_sequence_string, result)]));

        result
    }
}

impl Solution for Day21 {
    fn part1(&self, input: &str) -> String {
        let problem = Problem::parse_input(input, 2);
        let mut memo = HashMap::new();
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
            let sequence_length = problem.get_input_sequence(&mut memo, 0, code);

            let complexity_score = numeric_part as usize * sequence_length;
            result += complexity_score;
        }

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let problem = Problem::parse_input(input, 25);
        let mut memo = HashMap::new();
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
            let sequence_length = problem.get_input_sequence(&mut memo, 0, code);

            let complexity_score = numeric_part as usize * sequence_length;
            result += complexity_score;
        }

        result.to_string()
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
