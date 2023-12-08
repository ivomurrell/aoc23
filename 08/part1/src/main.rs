use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("../input.txt").expect("failed to open input file");
    let (instructions, network) = input.split_once("\n\n").expect("failed to parse input");
    let network: HashMap<_, _> = network
        .lines()
        .map(|line| {
            let (label, branches) = line.split_once(" = ").expect("failed to parse node");
            let (left, right) = branches.split_once(", ").expect("failed to parse branches");
            let left = &left[1..];
            let right = &right[..(right.len() - 1)];
            (label, (left, right))
        })
        .collect();
    let mut node = "AAA";
    let (step, _) = instructions
        .chars()
        .cycle()
        .enumerate()
        .find(|(_, instruction)| {
            let branches = network[&node];
            node = match instruction {
                'L' => branches.0,
                'R' => branches.1,
                _ => panic!("unexpected instruction"),
            };
            node == "ZZZ"
        })
        .expect("failed to reach ZZZ");
    let step_count = step + 1;
    println!("Took {step_count} steps to reach ZZZ!");
}
