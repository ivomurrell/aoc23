use std::fs::read_to_string;
use std::str;

fn main() {
    let calibration_sum: u32 = read_to_string("../input.txt")
        .expect("failed to open input file")
        .lines()
        .map(|line| {
            let first = line
                .bytes()
                .find(u8::is_ascii_digit)
                .expect("couldn't find a digit");
            let last = line
                .bytes()
                .rfind(u8::is_ascii_digit)
                .expect("couldn't find a second digit");
            str::from_utf8(&[first, last])
                .expect("failed to create string")
                .parse::<u32>()
                .expect("failed to parse number")
        })
        .sum();
    println!("The sum of all calibration values is {calibration_sum}!");
}
