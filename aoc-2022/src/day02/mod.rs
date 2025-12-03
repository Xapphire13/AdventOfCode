pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Tie,
}

fn get_losing_play(shape: Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Scissors,
        Shape::Paper => Shape::Rock,
        Shape::Scissors => Shape::Paper,
    }
}

fn get_winning_play(shape: Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Paper,
        Shape::Paper => Shape::Scissors,
        Shape::Scissors => Shape::Rock,
    }
}

fn get_shape_score(shape: Shape) -> u32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_opponent_shape(opponent_play: &str) -> Shape {
    match opponent_play {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Invalid play!"),
    }
}

fn get_my_shape(my_play: &str) -> Shape {
    match my_play {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("Invalid play!"),
    }
}

fn get_desired_outcome(input: &str) -> Outcome {
    match input {
        "X" => Outcome::Lose,
        "Y" => Outcome::Tie,
        "Z" => Outcome::Win,
        _ => panic!("Invalid input!"),
    }
}

fn play(opponent_shape: Shape, my_shape: Shape) -> u32 {
    let my_shape_val = get_shape_score(my_shape);
    let result = if get_losing_play(my_shape) == opponent_shape {
        6 // We win
    } else if get_losing_play(opponent_shape) == my_shape {
        0 // We lose
    } else {
        3 // Tie
    };

    result + my_shape_val
}

fn parse_input_line_v1(line: &str) -> (Shape, Shape) {
    let split: [&str; 2] = line.split(' ').collect::<Vec<&str>>().try_into().unwrap();

    (get_opponent_shape(split[0]), get_my_shape(split[1]))
}

fn parse_input_line_v2(line: &str) -> (Shape, Outcome) {
    let split: [&str; 2] = line.split(' ').collect::<Vec<&str>>().try_into().unwrap();

    (get_opponent_shape(split[0]), get_desired_outcome(split[1]))
}

fn part1(input: &Vec<String>) -> u32 {
    let mut score = 0;

    for line in input {
        let (opponent_play, my_play) = parse_input_line_v1(line);
        score += play(opponent_play, my_play);
    }

    score
}

fn part2(input: &Vec<String>) -> u32 {
    let mut score = 0;

    for line in input {
        let (opponent_play, desired_outcome) = parse_input_line_v2(line);

        match desired_outcome {
            Outcome::Lose => {
                let my_play = get_losing_play(opponent_play);
                score += play(opponent_play, my_play);
            }
            Outcome::Win => {
                let my_play = get_winning_play(opponent_play);
                score += play(opponent_play, my_play);
            }
            Outcome::Tie => score += play(opponent_play, opponent_play),
        };
    }

    score
}
