use shared::Solution;

pub struct Day7;

struct Test {
    result: u64,
    operands: Vec<u64>,
}

fn concat_op(left: u64, right: u64) -> u64 {
    format!("{}{}", left, right).parse().unwrap()
}

impl Test {
    fn is_valid(&self) -> bool {
        fn test(result: u64, acc: u64, curr: u64, next: &[u64]) -> bool {
            if acc > result {
                return false;
            }

            if next.is_empty() {
                return (acc + curr) == result || (acc * curr) == result;
            }

            if (acc + curr) <= result && test(result, acc + curr, next[0], &next[1..]) {
                return true;
            }

            if (acc * curr) <= result && test(result, acc * curr, next[0], &next[1..]) {
                return true;
            }

            false
        }

        test(self.result, 0, self.operands[0], &self.operands[1..])
    }

    fn is_valid2(&self) -> bool {
        fn test(result: u64, acc: u64, curr: u64, next: &[u64]) -> bool {
            if acc > result {
                return false;
            }

            if next.is_empty() {
                return (acc + curr) == result
                    || (acc * curr) == result
                    || concat_op(acc, curr) == result;
            }

            if (acc + curr) <= result && test(result, acc + curr, next[0], &next[1..]) {
                return true;
            }

            if (acc * curr) <= result && test(result, acc * curr, next[0], &next[1..]) {
                return true;
            }

            if concat_op(acc, curr) <= result
                && test(result, concat_op(acc, curr), next[0], &next[1..])
            {
                return true;
            }

            false
        }

        test(self.result, 0, self.operands[0], &self.operands[1..])
    }
}

struct Problem {
    tests: Vec<Test>,
}

impl Problem {
    fn parse_input(input: &str) -> Problem {
        let tests = input
            .trim()
            .lines()
            .map(|line| {
                let (result, rest) = line.split_once(": ").unwrap();
                let operands = rest
                    .split(" ")
                    .map(|operand| operand.parse().unwrap())
                    .collect();

                Test {
                    result: result.parse().unwrap(),
                    operands,
                }
            })
            .collect();

        Problem { tests }
    }
}

impl Solution for Day7 {
    fn part1(&self, input: &str) -> String {
        let problem = Problem::parse_input(input);

        let result: u64 = problem
            .tests
            .iter()
            .filter_map(|test| {
                if test.is_valid() {
                    Some(test.result)
                } else {
                    None
                }
            })
            .sum();

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let problem = Problem::parse_input(input);

        let result: u64 = problem
            .tests
            .iter()
            .filter_map(|test| {
                if test.is_valid2() {
                    Some(test.result)
                } else {
                    None
                }
            })
            .sum();

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use dedent::dedent;

    use super::*;

    #[test]
    fn test_day7() {
        let input = dedent!(
            "
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
            "
        );

        let result = Day7.part1(input);

        assert_eq!(result, "3749");
    }
}
