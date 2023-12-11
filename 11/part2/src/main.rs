use std::fs::read_to_string;

fn main() {
    let mut galaxies: Vec<_> = read_to_string("../input.txt")
        .expect("failed to open input file")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, space)| match space {
                    '#' => Some([x, y]),
                    _ => None,
                })
        })
        .collect();

    for dim in 0..=1 {
        galaxies.sort_by_key(|galaxy| galaxy[dim]);
        let mut empty_space = 0;
        let mut prev_dim = 0;
        for galaxy in galaxies.iter_mut() {
            let galaxy_dim = &mut galaxy[dim];
            empty_space += (*galaxy_dim - prev_dim).saturating_sub(1);
            prev_dim = *galaxy_dim;
            *galaxy_dim += empty_space * 999_999;
        }
    }

    let path_total = galaxies
        .iter()
        .flat_map(|galaxy_1| {
            galaxies.iter().map(|galaxy_2| {
                (0..=1)
                    .map(|dim| galaxy_1[dim].abs_diff(galaxy_2[dim]))
                    .sum::<usize>()
            })
        })
        .sum::<usize>()
        / 2;
    println!("The sum of the shortest paths between all galaxies is {path_total}!");
}
