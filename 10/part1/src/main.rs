use std::fs::read_to_string;

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

    let mut pipe_count = 1;
    let mut prev_pipe = start_coord;
    let mut current_pipe = starting_pipe;
    while current_pipe != start_coord {
        pipe_count += 1;
        let next_pipe = match get_adjacent_coords(current_pipe) {
            [pipe_1, pipe_2] if prev_pipe == pipe_1 => pipe_2,
            [pipe_1, _] => pipe_1,
        };
        prev_pipe = current_pipe;
        current_pipe = next_pipe;
    }

    let farthest_distance = pipe_count / 2;
    println!("The farthest pipe is {farthest_distance} steps away!");
}
