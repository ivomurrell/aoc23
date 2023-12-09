use std::fs::read_to_string;

fn main() {
    let extrapolated_sum: i32 = read_to_string("../input.txt")
        .expect("failed to open input file")
        .lines()
        .map(|line| {
            let mut differences: Vec<i32> = line
                .split_ascii_whitespace()
                .map(|val| val.parse().expect("failed to parse number"))
                .collect();
            let mut first_sum = 0;
            let mut should_negate = false;
            while !differences.iter().all(|&val| val == 0) {
                first_sum += differences.first().expect("list was empty")
                    * if should_negate { -1 } else { 1 };
                should_negate = !should_negate;
                differences = differences
                    .windows(2)
                    .map(|window| match window {
                        [x, y] => y - x,
                        _ => unreachable!(),
                    })
                    .collect();
            }
            first_sum
        })
        .sum();
    println!("The sum of all extrapolated values is {extrapolated_sum}!");
}
