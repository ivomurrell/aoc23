use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../input.txt").expect("failed to open input file");
    let engine_schematic: Vec<_> = input.lines().collect();
    let digits = Regex::new(r"\d+").expect("failed to compile regex");
    let mut gear_ratio_total = 0;
    for (row, line) in engine_schematic.iter().enumerate() {
        gear_ratio_total += line
            .chars()
            .enumerate()
            .filter_map(|(column, item)| match item {
                '*' => Some(column),
                _ => None,
            })
            .filter_map(|column| {
                let adjacent_rows =
                    row.saturating_sub(1)..=(row + 1).min(engine_schematic.len() - 1);
                let adjacent_columns =
                    column.saturating_sub(1)..=(column + 1).min(engine_schematic[0].len() - 1);
                let gear_numbers: Vec<_> = engine_schematic[adjacent_rows]
                    .iter()
                    .flat_map(|line| {
                        digits
                            .find_iter(line)
                            .filter(|num_match| {
                                adjacent_columns
                                    .clone()
                                    .any(|column| num_match.range().contains(&column))
                            })
                            .map(|num_match| {
                                num_match
                                    .as_str()
                                    .parse::<u32>()
                                    .expect("failed to parse gear number")
                            })
                    })
                    .collect();
                match gear_numbers.len() {
                    2 => Some(gear_numbers[0] * gear_numbers[1]),
                    _ => None,
                }
            })
            .sum::<u32>();
    }

    println!("The sum of all gear ratios is {gear_ratio_total}!");
}
