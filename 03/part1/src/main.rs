use regex::Regex;
use std::{fs::read_to_string, ops::Range};

fn main() {
    let input = read_to_string("../input.txt").expect("failed to open input file");
    let engine_schematic: Vec<_> = input.lines().collect();
    let is_engine_part = |row: usize, columns: Range<usize>| {
        let adjacent_rows = row.saturating_sub(1)..=(row + 1).min(engine_schematic.len() - 1);
        let adjacent_columns =
            columns.start.saturating_sub(1)..(columns.end + 1).min(engine_schematic[0].len() - 1);
        engine_schematic[adjacent_rows].iter().any(|line| {
            line[adjacent_columns.clone()]
                .bytes()
                .any(|item| item != b'.' && !item.is_ascii_digit())
        })
    };
    let digits = Regex::new(r"\d+").expect("failed to compile regex");
    let mut part_number_total = 0;
    for (row, line) in engine_schematic.iter().enumerate() {
        for part_number_match in digits.find_iter(line) {
            if is_engine_part(row, part_number_match.range()) {
                part_number_total += part_number_match
                    .as_str()
                    .parse::<u32>()
                    .expect("failed to parse part number");
            }
        }
    }

    println!("The sum of all part numbers is {part_number_total}!");
}
