use std::{fs::read_to_string, iter};

fn main() {
    let input = read_to_string("../input.txt").expect("failed to open input file");
    let mut numbers = input.lines().map(|line| {
        let (_, numbers) = line.split_once(':').expect("failed to parse line");
        numbers
            .trim_start()
            .split_ascii_whitespace()
            .map(|number| number.parse::<u32>().expect("failed to parse number"))
    });
    let races = iter::zip(
        numbers.next().expect("missing times"),
        numbers.next().expect("missing distances"),
    );
    let margin_of_error: usize = races
        .map(|(time, record)| {
            (0..=time)
                .filter(|held_time| {
                    let distance = held_time * (time - held_time);
                    distance > record
                })
                .count()
        })
        .product();
    println!("Your margin of error is {margin_of_error}!");
}
