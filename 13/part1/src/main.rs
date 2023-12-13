use std::fs::read_to_string;

fn find_reflections(pattern: &Vec<Vec<char>>) -> Option<usize> {
    let mut possible_reflections: Vec<_> = (1..pattern[0].len()).collect();
    for line in pattern {
        possible_reflections.retain(|&reflection| {
            let (before, after) = line.split_at(reflection);
            before
                .iter()
                .rev()
                .zip(after.iter())
                .all(|(before, after)| before == after)
        })
    }
    possible_reflections.first().copied()
}

fn main() {
    let reflection_summary: usize = read_to_string("../input.txt")
        .expect("failed to open input file")
        .split("\n\n")
        .map(|pattern| {
            let pattern = pattern.lines().map(|line| line.chars().collect()).collect();
            match find_reflections(&pattern) {
                Some(reflection) => reflection,
                None => {
                    let transposed = (0..pattern[0].len())
                        .map(|i| pattern.iter().map(|inner| inner[i]).collect())
                        .collect();
                    find_reflections(&transposed).expect("failed to find any reflection") * 100
                }
            }
        })
        .sum();
    println!("The summarized note of reflections is {reflection_summary}!")
}
