use std::fs::read_to_string;

fn main() {
    let hashed_sum: u32 = read_to_string("../input.txt")
        .expect("failed to open input file")
        .trim_end()
        .split(',')
        .map(|step| {
            step.bytes().fold(0, |current_value, ch| {
                ((current_value + ch as u32) * 17) % 256
            })
        })
        .sum();
    println!("The sum of all HASH'd values is {hashed_sum}!");
}
