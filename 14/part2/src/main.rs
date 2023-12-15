use std::{
    collections::{hash_map::Entry, HashMap},
    fs::read_to_string,
    ops::ControlFlow,
};

fn main() {
    let input = read_to_string("../input.txt").expect("failed to open input file");
    let line_length = input.lines().next().expect("input was empty").len();
    let input_height = input.lines().count();

    let north: (Vec<usize>, Vec<usize>) = {
        (
            (0..input_height)
                .map(|row| row * (line_length + 1))
                .collect(),
            (0..line_length).collect(),
        )
    };
    let west: (Vec<usize>, Vec<usize>) = {
        (
            (0..input_height).collect(),
            (0..line_length)
                .map(|col| col * (input_height + 1))
                .collect(),
        )
    };
    let south = {
        let (mut rows, mut columns) = north.clone();
        rows.reverse();
        columns.reverse();
        (rows, columns)
    };
    let east = {
        let (mut rows, mut columns) = west.clone();
        rows.reverse();
        columns.reverse();
        (rows, columns)
    };
    let spin_cycle = [north, west, south, east];

    let mut platform = input.into_bytes();
    let mut cycled_platforms = HashMap::new();
    let total_cycles = 1_000_000_000;
    let cycle_control_flow = (0..total_cycles).try_for_each(|spin_count| {
        for (rows, columns) in spin_cycle.iter() {
            for column in columns {
                (0..rows.len()).fold(0, |max_idx, row_idx| {
                    let index = column + rows[row_idx];
                    match platform[index] {
                        b'#' => row_idx + 1,
                        b'O' => {
                            platform.swap(index, column + rows[max_idx]);
                            max_idx + 1
                        }
                        _ => max_idx,
                    }
                });
            }
        }
        match cycled_platforms.entry(platform.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(spin_count);
                ControlFlow::Continue(())
            }
            Entry::Occupied(prev_count) => ControlFlow::Break((spin_count, *prev_count.get())),
        }
    });
    let final_platform = match cycle_control_flow {
        ControlFlow::Break((spin_count, prev_count)) => {
            let spin_index =
                (total_cycles - 1 - prev_count) % (spin_count - prev_count) + prev_count;
            cycled_platforms
                .into_iter()
                .find(|&(_, count)| count == spin_index)
                .expect("failed to find previously looped cycle")
                .0
        }
        _ => platform,
    };

    let total_load = final_platform
        .into_iter()
        .fold((0, input_height), |(total, load), tile| match tile {
            b'O' => (total + load, load),
            b'\n' => (total, load - 1),
            _ => (total, load),
        })
        .0;
    println!("The total load on all support beams after a billion cycles is {total_load}!");
}
