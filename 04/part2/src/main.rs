use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let mut card_count = HashMap::new();
    let card_total: u32 = read_to_string("../input.txt")
        .expect("failed to open input file")
        .lines()
        .map(|card| {
            let (card_id, numbers) = card.split_once(": ").expect("failed to parse card");
            let card_num: usize = card_id
                .split_whitespace()
                .nth(1)
                .expect("failed to parse card id")
                .parse()
                .expect("failed to parse card number");
            let current_card_count = card_count.get(&card_num).copied().unwrap_or(1);
            let (winning_numbers, your_numbers) =
                numbers.split_once(" | ").expect("failed to parse numbers");
            let winning_numbers: HashSet<_> = winning_numbers.split_whitespace().collect();
            let winning_number_count = your_numbers
                .split_whitespace()
                .filter(|number| winning_numbers.contains(number))
                .count();
            if winning_number_count > 0 {
                for copy_num in (card_num + 1)..(card_num + 1 + winning_number_count) {
                    *card_count.entry(copy_num).or_insert(1) += current_card_count;
                }
            }
            current_card_count
        })
        .sum();
    println!("You win a total of {card_total} scratchcards!");
}
