use shared::Solution;

pub struct Day6;

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Problem {
    values: Vec<usize>,
    operator: Operator,
}

#[derive(Debug)]
struct ProblemSet {
    problems: Vec<Problem>,
}

impl Solution for Day6 {
    fn part1(&self, input: &str) -> String {
        let problem_set = ProblemSet::new(input);

        problem_set.grand_total().to_string()
    }

    fn part2(&self, input: &str) -> String {
        String::from("todo")
    }
}

impl Operator {
    fn parse(value: &str) -> Self {
        match value {
            "*" => Operator::Multiply,
            _ => Operator::Add,
        }
    }
}

impl Problem {
    fn solve(&self) -> usize {
        let init = match self.operator {
            Operator::Add => 0,
            Operator::Multiply => 1,
        };

        self.values
            .iter()
            .fold(init, |acc, curr| match self.operator {
                Operator::Add => acc + curr,
                Operator::Multiply => acc * curr,
            })
    }
}

impl ProblemSet {
    fn new(input: &str) -> Self {
        let mut problems = vec![];

        for line in input.trim().lines() {
            let tokens = line.split_whitespace().collect::<Vec<_>>();

            if problems.is_empty() {
                problems.resize_with(tokens.len(), || Problem {
                    values: vec![],
                    operator: Operator::Add,
                });
            }

            for (index, &token) in tokens.iter().enumerate() {
                match token.parse::<usize>() {
                    Ok(value) => problems[index].values.push(value),
                    Err(_) => problems[index].operator = Operator::parse(token),
                }
            }
        }

        Self { problems }
    }

    fn grand_total(&self) -> usize {
        self.problems.iter().map(|problem| problem.solve()).sum()
    }
}
