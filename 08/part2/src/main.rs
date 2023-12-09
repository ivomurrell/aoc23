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
    let shortest_steps: Vec<_> = network
        .keys()
        .cloned()
        .filter(|label| label.ends_with('A'))
        .map(|start| {
            let mut node = start;
            instructions
                .chars()
                .cycle()
                .enumerate()
                .find(|(_, instruction)| {
                    let branches = network[node];
                    node = match instruction {
                        'L' => branches.0,
                        'R' => branches.1,
                        _ => panic!("unexpected instruction"),
                    };
                    node.ends_with('Z')
                })
                .expect("failed to reach nodes ending in Z")
                .0
                + 1
        })
        .collect();
    let mut lcm_list = shortest_steps.clone();
    while !lcm_list.iter().all(|&val| val == lcm_list[0]) {
        let (min_index, _) = lcm_list
            .iter()
            .enumerate()
            .min_by_key(|(_, &val)| val)
            .expect("list was empty");
        lcm_list[min_index] += shortest_steps[min_index];
    }
    let lcm_steps = lcm_list[0];
    println!("Took {lcm_steps} steps to all reach nodes ending in Z!");
}
