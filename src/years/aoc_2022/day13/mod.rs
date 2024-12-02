use std::{cmp::Ordering, fmt::Debug, str::Chars, vec};

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Clone, Debug)]
enum ListItem {
    Integer(i32),
    List(Vec<ListItem>),
}

impl PartialEq for ListItem {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for ListItem {}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (ListItem::Integer(left), ListItem::Integer(right)) => left.cmp(right),
            (ListItem::List(left), ListItem::List(right)) => {
                let max_index = left.len().min(right.len());

                for i in 0..max_index {
                    let left_child = &left[i];
                    let right_child = &right[i];

                    match left_child.cmp(right_child) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        _ => {}
                    }
                }

                left.len().cmp(&right.len())
            }
            (ListItem::Integer(left), ListItem::List(right)) => {
                ListItem::List(vec![ListItem::Integer(*left)]).cmp(&ListItem::List(right.clone()))
            }
            (ListItem::List(left), ListItem::Integer(right)) => {
                ListItem::List(left.clone()).cmp(&ListItem::List(vec![ListItem::Integer(*right)]))
            }
        }
    }
}

fn parse_list(char_iter: &mut Chars) -> Vec<ListItem> {
    let mut res = vec![];
    let mut temp_str: Option<String> = None;

    while let Some(character) = char_iter.next() {
        match character {
            '[' => {
                res.push(ListItem::List(parse_list(char_iter)));
            }
            '0'..='9' => {
                if let Some(temp_str) = &mut temp_str {
                    temp_str.push(character);
                } else {
                    temp_str = Some(String::from(character));
                }
            }
            ',' | ']' => {
                if let Some(unwrapped) = temp_str {
                    res.push(ListItem::Integer(unwrapped.parse().unwrap()));
                    temp_str = None;
                }

                if character == ']' {
                    break;
                }
            }
            _ => {}
        }
    }

    res
}

fn parse_line(line: &str) -> ListItem {
    let mut chars = line.chars();
    chars.next(); // Skip first '['

    ListItem::List(parse_list(&mut chars))
}

fn parse_input(input: &[String]) -> Vec<(ListItem, ListItem)> {
    let mut iter = input.iter();
    let mut result = vec![];

    while let Some(left) = iter.next() {
        let right = iter.next().unwrap();
        iter.next(); // Consume empty line

        result.push((parse_line(left), parse_line(right)));
    }

    result
}

fn part1(input: &[String]) -> u32 {
    let pairs = parse_input(input);
    let mut res = 0;

    for (i, (left, right)) in pairs.iter().enumerate() {
        if left <= right {
            res += (i + 1) as u32;
        }
    }

    res
}

fn part2(input: &[String]) -> u32 {
    let pairs = parse_input(input);
    let divider_1 = ListItem::List(vec![ListItem::List(vec![ListItem::Integer(2)])]);
    let divider_2 = ListItem::List(vec![ListItem::List(vec![ListItem::Integer(6)])]);
    let mut packets = pairs
        .into_iter()
        .flat_map(|(left, right)| vec![left, right])
        .collect::<Vec<ListItem>>();
    packets.push(divider_1.clone());
    packets.push(divider_2.clone());
    packets.sort();

    let divider_1_index = packets.iter().position(|it| *it == divider_1).unwrap() + 1;
    let divider_2_index = packets.iter().position(|it| *it == divider_2).unwrap() + 1;

    (divider_1_index * divider_2_index) as u32
}
