use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

type Coords = (usize, usize);
type EnergisedTiles = HashMap<Coords, HashSet<Direction>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn advance_coords((x, y): Coords, direction: Direction) -> Option<Coords> {
    Some(match direction {
        Direction::Up => (x, y.checked_sub(1)?),
        Direction::Right => (x + 1, y),
        Direction::Down => (x, y + 1),
        Direction::Left => (x.checked_sub(1)?, y),
    })
}

fn split_beam(
    grid: &Vec<Vec<char>>,
    energised: &mut EnergisedTiles,
    coords: Coords,
    split: [Direction; 2],
) {
    split
        .into_iter()
        .filter_map(|direction| Some((advance_coords(coords, direction)?, direction)))
        .for_each(|(coords, direction)| follow_beam(grid, energised, coords, direction));
}

fn follow_beam(
    grid: &Vec<Vec<char>>,
    energised: &mut EnergisedTiles,
    start: Coords,
    start_direction: Direction,
) {
    let mut coords = start;
    let mut direction = start_direction;
    while let Some(tile) = grid.get(coords.1).and_then(|column| column.get(coords.0)) {
        if !energised.entry(coords).or_default().insert(direction) {
            return;
        }
        match (tile, direction) {
            ('|', Direction::Left | Direction::Right) => {
                split_beam(grid, energised, coords, [Direction::Up, Direction::Down]);
                return;
            }
            ('-', Direction::Up | Direction::Down) => {
                split_beam(grid, energised, coords, [Direction::Left, Direction::Right]);
                return;
            }
            ('/', _) => {
                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                };
            }
            ('\\', _) => {
                direction = match direction {
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                };
            }
            _ => {}
        };
        match advance_coords(coords, direction) {
            Some(next) => {
                coords = next;
            }
            None => {
                return;
            }
        }
    }
}

fn main() {
    let grid: Vec<Vec<_>> = read_to_string("../input.txt")
        .expect("failed to open input file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut energised_tiles = HashMap::new();
    follow_beam(&grid, &mut energised_tiles, (0, 0), Direction::Right);
    let energised_count = energised_tiles.len();
    println!("A total of {energised_count} tiles ended up being energised!");
}
