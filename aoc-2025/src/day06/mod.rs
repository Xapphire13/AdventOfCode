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
        let problem_set = ProblemSet::new_alt(input);

        problem_set.grand_total().to_string()
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

    fn new_alt(input: &str) -> Self {
        let mut problems = vec![];
        let mut lines = input.trim().lines().collect::<Vec<_>>();
        let operator_line = *lines.last().unwrap();
        let mut problem_widths = vec![];

        // Figure out the width of each problem
        let mut problem_width = 0usize;
        for c in operator_line.chars() {
            if !c.is_whitespace() {
                if problem_width > 0 {
                    problem_widths.push(problem_width - 1);
                }

                problem_width = 1;

                problems.push(Problem {
                    operator: Operator::parse(&c.to_string()),
                    values: vec![],
                });
            } else {
                problem_width += 1;
            }
        }

        let longest_line = lines.iter().map(|line| line.len()).max().unwrap();
        let last_problem_width = longest_line - operator_line.len() + 1;
        problem_widths.push(last_problem_width);

        lines.pop(); // Remove the operator line

        // For each problem, get the constituent numbers
        let mut col = 0;
        for (index, &width) in problem_widths.iter().enumerate() {
            let problem = &mut problems[index];

            for _ in 0..width {
                let number = ProblemSet::get_vertical_number(&lines, col);
                problem.values.push(number);
                col += 1;
            }

            col += 1;
        }

        Self { problems }
    }

    fn grand_total(&self) -> usize {
        self.problems.iter().map(|problem| problem.solve()).sum()
    }

    fn get_vertical_number(data: &Vec<&str>, col: usize) -> usize {
        let mut buf = vec![];

        for &row in data {
            let c = row.chars().nth(col).unwrap_or('0').to_string();
            buf.push(c);
        }

        buf.join("").trim().parse().unwrap_or(0)
    }
}
