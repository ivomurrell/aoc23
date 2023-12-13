use std::fs::read_to_string;

fn find_reflections(pattern: &Vec<Vec<char>>) -> Option<usize> {
    let mut possible_reflections: Vec<_> = (1..pattern[0].len())
        .map(|reflection| (reflection, false))
        .collect();
    for line in pattern {
        possible_reflections.retain_mut(|(reflection, is_smudged)| {
            let (before, after) = line.split_at(*reflection);
            match before
                .iter()
                .rev()
                .zip(after.iter())
                .filter(|(before, after)| before != after)
                .count()
            {
                0 => true,
                1 if !*is_smudged => {
                    *is_smudged = true;
                    true
                }
                _ => false,
            }
        })
    }
    possible_reflections
        .into_iter()
        .find(|&(_, is_smudged)| is_smudged)
        .map(|(reflection, _)| reflection)
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
