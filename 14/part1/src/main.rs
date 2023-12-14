use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../input.txt").expect("failed to open input file");
    let line_length = input.lines().next().expect("input was empty").len();
    let input_height = input.lines().count();
    let total_load: usize = (0..line_length)
        .map(|column| {
            let mut max_north = input_height;
            input
                .lines()
                .enumerate()
                .map(|(height, row)| {
                    let height = input_height - height;
                    match row
                        .chars()
                        .nth(column)
                        .expect("row was shorter than others")
                    {
                        '#' => {
                            max_north = height - 1;
                            0
                        }
                        'O' => {
                            max_north -= 1;
                            max_north + 1
                        }
                        _ => 0,
                    }
                })
                .sum::<usize>()
        })
        .sum();
    println!("The total load on all support beams is {total_load}!");
}
