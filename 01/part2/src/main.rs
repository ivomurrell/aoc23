use std::fs::read_to_string;

fn check_for_number(slice: &str) -> Option<u32> {
    let first_char = slice.chars().next().expect("string was empty");
    match first_char {
        c if c.is_ascii_digit() => Some(first_char.to_digit(10).unwrap()),
        'o' if slice.starts_with("one") => Some(1),
        't' if slice.starts_with("two") => Some(2),
        't' if slice.starts_with("three") => Some(3),
        'f' if slice.starts_with("four") => Some(4),
        'f' if slice.starts_with("five") => Some(5),
        's' if slice.starts_with("six") => Some(6),
        's' if slice.starts_with("seven") => Some(7),
        'e' if slice.starts_with("eight") => Some(8),
        'n' if slice.starts_with("nine") => Some(9),
        _ => None,
    }
}

fn main() {
    let calibration_sum: u32 = read_to_string("../input.txt")
        .expect("failed to open input file")
        .lines()
        .map(|line| {
            let first = (0..line.len())
                .find_map(|i| check_for_number(&line[i..]))
                .expect("couldn't find a number");
            let last = (0..line.len())
                .rev()
                .find_map(|i| check_for_number(&line[i..]))
                .expect("couldn't find a second number");
            first * 10 + last
        })
        .sum();
    println!("The sum of all calibration values is {calibration_sum}!");
}
