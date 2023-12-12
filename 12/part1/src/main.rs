use std::{collections::BTreeSet, fs::read_to_string};

use itertools::Itertools;

fn main() {
    let arrangement_count: usize = read_to_string("../input.txt")
        .expect("failed to open input file")
        .lines()
        .map(|row| {
            let (springs, groups) = row.split_once(' ').expect("failed to parse row");
            let groups: Vec<usize> = groups
                .split(',')
                .map(|group| group.parse().expect("failed to parse group"))
                .collect();
            let damaged: BTreeSet<_> = springs.match_indices('#').map(|(index, _)| index).collect();
            let missing_damaged = groups.iter().sum::<usize>() - damaged.len();
            springs
                .match_indices('?')
                .map(|(index, _)| index)
                .combinations(missing_damaged)
                .map(|damaged_guess| {
                    let mut row_guess = damaged.clone();
                    row_guess.extend(damaged_guess);
                    let mut guessed_groups = Vec::new();
                    let mut prev_spring = 0;
                    let mut current_group = 0;
                    for spring in row_guess {
                        if spring != prev_spring + 1 && current_group != 0 {
                            guessed_groups.push(current_group);
                            current_group = 0;
                        }
                        current_group += 1;
                        prev_spring = spring;
                    }
                    guessed_groups.push(current_group);
                    guessed_groups
                })
                .filter(|guess| guess == &groups)
                .count()
        })
        .sum();
    println!("The total number of different row arrangements is {arrangement_count}!");
}
