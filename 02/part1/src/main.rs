use std::fs::read_to_string;

fn main() {
    let id_sum: u32 = read_to_string("../input.txt")
        .expect("failed to open input file")
        .lines()
        .filter_map(|line| {
            let (game_id, game_log) = line.split_once(':').expect("failed to parse line");
            let possible = game_log.split(';').all(|reveals| {
                reveals.split(',').map(str::trim_start).all(|reveal| {
                    let (count, colour) = reveal.split_once(' ').expect("failed to parse reveal");
                    let max_count = match colour {
                        "red" => 12,
                        "green" => 13,
                        "blue" => 14,
                        _ => 0,
                    };
                    count.parse::<u32>().expect("failed to parse count") <= max_count
                })
            });
            if possible {
                Some(
                    game_id
                        .split_once(' ')
                        .expect("failed to parse ID")
                        .1
                        .parse::<u32>()
                        .expect("failed to parse ID number"),
                )
            } else {
                None
            }
        })
        .sum();
    println!("The sum of all possible IDs is {id_sum}!");
}
