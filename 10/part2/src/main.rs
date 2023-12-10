use std::{cmp::Ordering, collections::HashSet, fs::read_to_string};

#[cfg(feature = "visualise")]
use colored::*;

type Coord = (usize, usize);

fn north((x, y): Coord) -> Coord {
    (x, y - 1)
}

fn east((x, y): Coord) -> Coord {
    (x + 1, y)
}

fn south((x, y): Coord) -> Coord {
    (x, y + 1)
}

fn west((x, y): Coord) -> Coord {
    (x - 1, y)
}

fn main() {
    let grid: Vec<Vec<_>> = read_to_string("../input.txt")
        .expect("failed to open input file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let get_adjacent_coords = |pipe_coord @ (pipe_x, pipe_y): Coord| -> [Coord; 2] {
        let pipe = grid[pipe_y][pipe_x];
        match pipe {
            '|' => [north(pipe_coord), south(pipe_coord)],
            '-' => [east(pipe_coord), west(pipe_coord)],
            'L' => [north(pipe_coord), east(pipe_coord)],
            'J' => [north(pipe_coord), west(pipe_coord)],
            '7' => [south(pipe_coord), west(pipe_coord)],
            'F' => [south(pipe_coord), east(pipe_coord)],
            _ => panic!("unrecognised pipe"),
        }
    };

    let start_coord @ (start_x, start_y) = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| line.iter().position(|&tile| tile == 'S').map(|x| (x, y)))
        .expect("failed to find start");

    let mut starting_adjacent_tiles = Vec::new();
    if start_y > 0 {
        starting_adjacent_tiles.push(north(start_coord));
    }
    if start_x < grid[0].len() - 1 {
        starting_adjacent_tiles.push(east(start_coord));
    }
    if start_y < grid.len() - 1 {
        starting_adjacent_tiles.push(south(start_coord));
    }
    if start_x > 0 {
        starting_adjacent_tiles.push(west(start_coord));
    }
    let starting_pipe = starting_adjacent_tiles
        .into_iter()
        .filter(|&(tile_x, tile_y)| grid[tile_y][tile_x] != '.')
        .find(|&pipe| get_adjacent_coords(pipe).contains(&start_coord))
        .expect("failed to find adjacent pipe to start");

    let mut pipe_loop = vec![start_coord];
    let mut prev_pipe = start_coord;
    let mut current_pipe = starting_pipe;
    while current_pipe != start_coord {
        pipe_loop.push(current_pipe);
        let next_pipe = match get_adjacent_coords(current_pipe) {
            [pipe_1, pipe_2] if prev_pipe == pipe_1 => pipe_2,
            [pipe_1, _] => pipe_1,
        };
        prev_pipe = current_pipe;
        current_pipe = next_pipe;
    }

    let mut enclosed_tiles = HashSet::new();
    pipe_loop
        .windows(2)
        .map(|window| (window[0], window[1]))
        .map(|(prev @ (prev_x, prev_y), current @ (curr_x, curr_y))| {
            let direction = match (prev_x.cmp(&curr_x), prev_y.cmp(&curr_y)) {
                (Ordering::Equal, Ordering::Greater) => |(x, y)| (x + 1, y),
                (Ordering::Less, Ordering::Equal) => |(x, y)| (x, y + 1),
                (Ordering::Equal, Ordering::Less) => |(x, y)| (x - 1, y),
                (Ordering::Greater, Ordering::Equal) => |(x, y)| (x, y - 1),
                _ => panic!("unexpected direction"),
            };
            (prev, current, direction)
        })
        .for_each(|(prev, current, direction)| {
            for start in [prev, current] {
                let mut tile = direction(start);
                while !pipe_loop.contains(&tile) {
                    enclosed_tiles.insert(tile);
                    tile = direction(tile);
                }
            }
        });

    #[cfg(feature = "visualise")]
    for (y, line) in grid.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            let tile = tile.to_string();
            let coloured_tile = if start_coord == (x, y) {
                tile.bright_black().bold().italic()
            } else if pipe_loop.contains(&(x, y)) {
                tile.bright_black()
            } else if enclosed_tiles.contains(&(x, y)) {
                tile.red()
            } else {
                tile.normal()
            };
            print!("{coloured_tile}");
        }
        println!();
    }

    let enclosed_count = enclosed_tiles.len();
    println!("The number of tiles enclosed by the loop is {enclosed_count}!")
}
