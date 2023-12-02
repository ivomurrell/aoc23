use std::fs::read_to_string;

fn main() {
    let power_sum: u32 = read_to_string("../input.txt")
        .expect("failed to open input file")
        .lines()
        .map(|line| {
            let (_, game_log) = line.split_once(':').expect("failed to parse line");
            let mut min_cubes = [0, 0, 0];
            for reveal in game_log.split([';', ',']).map(str::trim_start) {
                let (count, colour) = reveal.split_once(' ').expect("failed to parse reveal");
                let colour_index = match colour {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => panic!("unexpected cube colour"),
                };
                let min_colour = &mut min_cubes[colour_index];
                let count: u32 = count.parse().expect("failed to parse cube count");
                *min_colour = count.max(*min_colour);
            }
            min_cubes.into_iter().product::<u32>()
        })
        .sum();
    println!("The sum of all powers is {power_sum}!");
}
