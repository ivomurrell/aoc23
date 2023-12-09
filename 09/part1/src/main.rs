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
            let mut last_sum = 0;
            while !differences.iter().all(|&val| val == 0) {
                last_sum += differences.last().expect("list was empty");
                differences = differences
                    .windows(2)
                    .map(|window| match window {
                        [x, y] => y - x,
                        _ => unreachable!(),
                    })
                    .collect();
            }
            last_sum
        })
        .sum();
    println!("The sum of all extrapolated values is {extrapolated_sum}!");
}
