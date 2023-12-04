use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let point_total: u32 = read_to_string("../input.txt")
        .expect("failed to open input file")
        .lines()
        .map(|card| {
            let (_, numbers) = card.split_once(": ").expect("failed to parse card");
            let (winning_numbers, your_numbers) =
                numbers.split_once(" | ").expect("failed to parse numbers");
            let winning_numbers: HashSet<_> = winning_numbers.split_whitespace().collect();
            let winning_number_count = your_numbers
                .split_whitespace()
                .filter(|number| winning_numbers.contains(number))
                .count();
            match winning_number_count {
                0 => 0,
                count => 1 << (count - 1),
            }
        })
        .sum();
    println!("The scratchcards are worth a total of {point_total}!");
}
