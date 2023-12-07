use std::{
    collections::{BTreeMap, HashMap},
    fs::read_to_string,
};

fn main() {
    let hands: BTreeMap<_, _> = read_to_string("../input.txt")
        .expect("failed to open input file")
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').expect("failed to parse line");
            let mut freq = HashMap::<_, u32>::new();
            for card in hand.chars() {
                *freq.entry(card).or_default() += 1u32;
            }
            let score = match freq.values().max().expect("hand was empty") {
                5 => 6,
                4 => 5,
                3 if freq.values().any(|&freq| freq == 2) => 4,
                3 => 3,
                2 if freq.values().filter(|&freq| *freq == 2).count() == 2 => 2,
                2 => 1,
                _ => 0,
            };
            (
                (
                    score,
                    hand.replace('A', "E")
                        .replace('T', "A")
                        .replace('J', "B")
                        .replace('Q', "C")
                        .replace('K', "D"),
                ),
                bid.parse::<usize>().expect("failed to parse bid"),
            )
        })
        .collect();
    let total_winnings: usize = hands
        .values()
        .enumerate()
        .map(|(rank, bid)| (rank + 1) * bid)
        .sum();
    println!("The total winnings are {total_winnings}!");
}
