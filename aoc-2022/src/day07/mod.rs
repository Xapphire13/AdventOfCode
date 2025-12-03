use std::{cell::RefCell, rc::Rc, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

enum Command {
    ChangeDir(String),
    List,
}

#[derive(Clone)]
struct File {
    name: String,
    is_directory: bool,
    children: Vec<Rc<RefCell<File>>>,
    size: usize,
    parent: Option<Rc<RefCell<File>>>,
}

fn parse_command(command: &str) -> Command {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\$ (\S+)( \S+)?").unwrap();
    }

    let caps = RE.captures(command).unwrap();
    let command_text = caps.get(1).unwrap().as_str();
    let args = caps.get(2).map(|value| value.as_str());

    match command_text {
        "cd" => Command::ChangeDir(String::from_str(args.unwrap().trim()).unwrap()),
        _ => Command::List,
    }
}

fn count_files(result: &mut Vec<usize>, file: &File) -> usize {
    let size = if file.is_directory {
        let mut sum = 0;
        file.children
            .iter()
            .for_each(|it| sum += count_files(result, &it.borrow()));

        sum
    } else {
        file.size
    };

    if file.is_directory {
        result.push(size);
    }

    size
}

fn parse_file(file: &str, parent: Rc<RefCell<File>>) -> File {
    let mut iter = file.split(' ');
    let dir_or_size = iter.next().unwrap();
    let name = iter.next().unwrap();

    if dir_or_size == "dir" {
        File {
            name: String::from_str(name).unwrap(),
            children: vec![],
            size: 0,
            is_directory: true,
            parent: Some(parent),
        }
    } else {
        File {
            name: String::from_str(name).unwrap(),
            children: vec![],
            size: dir_or_size.parse::<usize>().unwrap(),
            is_directory: false,
            parent: Some(parent),
        }
    }
}

fn build_tree(input: &[String]) -> Rc<RefCell<File>> {
    let root = File {
        name: String::from("/"),
        size: 0,
        children: vec![],
        is_directory: true,
        parent: None,
    };
    let root = Rc::new(RefCell::new(root));
    let mut cwd = root.clone();
    let mut iter = input.iter();

    while let Some(line) = iter.next() {
        if line.starts_with('$') {
            match parse_command(line) {
                Command::ChangeDir(arg) => match arg.as_str() {
                    "/" => {
                        cwd = root.clone();
                    }
                    ".." => {
                        let parent = cwd.borrow().parent.as_ref().cloned().unwrap();
                        cwd = parent;
                    }
                    _ => {
                        let child = cwd
                            .borrow()
                            .children
                            .iter()
                            .find(|it| it.borrow().name == arg)
                            .unwrap()
                            .clone();
                        cwd = child;
                    }
                },
                Command::List => {
                    let ls_iter = iter.clone().take_while(|it| !it.starts_with('$'));
                    for line in ls_iter {
                        let file = parse_file(line, cwd.clone());
                        cwd.borrow_mut().children.push(Rc::new(RefCell::new(file)));
                        iter.next(); // Advance overall iterator
                    }
                }
            }
        }
    }

    root
}

fn part1(input: &[String]) -> usize {
    let root = build_tree(input);

    let mut result = 0;
    let mut small_files = vec![];
    count_files(&mut small_files, &root.borrow());

    for val in small_files {
        if val < 100000 {
            result += val;
        }
    }

    result
}

fn part2(input: &[String]) -> usize {
    let root = build_tree(input);

    let mut small_files = vec![];
    let used_space = count_files(&mut small_files, &root.borrow());
    small_files.sort();
    let free_space = 70000000 - used_space;

    for val in small_files {
        if (free_space + val) >= 30000000 {
            return val;
        }
    }

    0
}
