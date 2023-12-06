use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../input.txt").expect("failed to open input file");
    let mut numbers = input.lines().map(|line| {
        let (_, numbers) = line.split_once(':').expect("failed to parse line");
        numbers
            .trim_start()
            .split_ascii_whitespace()
            .collect::<String>()
            .parse::<u64>()
            .expect("failed to parse number")
    });
    let (time, record) = (
        numbers.next().expect("missing times"),
        numbers.next().expect("missing distances"),
    );
    let margin_of_error = (0..=time)
        .filter(|held_time| {
            let distance = held_time * (time - held_time);
            distance > record
        })
        .count();
    println!("Your margin of error is {margin_of_error}!");
}
