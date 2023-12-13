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
                    row_guess
                        .into_iter()
                        .map(|spring| (spring, 1))
                        .coalesce(|prev, current| {
                            if current.0 == prev.0 + 1 {
                                Ok((current.0, prev.1 + 1))
                            } else {
                                Err((prev, current))
                            }
                        })
                        .map(|(_, count)| count)
                        .collect()
                })
                .filter(|guess: &Vec<_>| guess == &groups)
                .count()
        })
        .sum();
    println!("The total number of different row arrangements is {arrangement_count}!");
}
