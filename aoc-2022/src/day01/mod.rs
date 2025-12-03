pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let mut max = 0;
    let mut sum = 0;

    for line in input.iter() {
        if line.is_empty() {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    if sum > max {
        max = sum;
    }

    max
}

fn part2(input: &[String]) -> i32 {
    let mut top3 = [0; 3];

    let mut insert = |val: i32| {
        // Find insertion point, shift values, insert
        if val < top3[0] {
            // NO-OP
        } else if val < top3[1] {
            top3[0] = val;
        } else if val < top3[2] {
            top3[0] = top3[1];
            top3[1] = val;
        } else {
            top3[0] = top3[1];
            top3[1] = top3[2];
            top3[2] = val;
        }
    };

    let mut sum = 0;

    for line in input.iter() {
        if line.is_empty() {
            insert(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    insert(sum);

    top3.iter().sum()
}
