use std::{
    collections::{BTreeMap, HashMap},
    fs::read_to_string,
};

struct AlmanacMap {
    range_length: u32,
    dest_start: u32,
}

struct AlmanacMapping {
    dest_category: String,
    maps: BTreeMap<u32, AlmanacMap>,
}

fn main() {
    let almanac = read_to_string("../input.txt").expect("failed to open input file");
    let (seeds, mappings) = almanac.split_once("\n\n").expect("failed to parse almanac");
    let seeds: Vec<u32> = seeds
        .strip_prefix("seeds: ")
        .expect("failed to parse seeds")
        .split_whitespace()
        .map(|seed| seed.parse().expect("failed to parse seed"))
        .collect();
    let mut almanac_mappings = HashMap::new();
    for mapping in mappings.split("\n\n") {
        let (category_identifier, maps) =
            mapping.split_once('\n').expect("failed to parse mapping");
        let (src_category, dest_category) = category_identifier
            .split_once("-to-")
            .expect("failed to parse category identifier");
        let dest_category = dest_category
            .strip_suffix(" map:")
            .expect("failed to parse destination category");
        let maps = maps
            .lines()
            .map(|map| {
                let mut values = map
                    .split_whitespace()
                    .map(|value| value.parse().expect("failed to parse mapping number"));
                let dest_start: u32 = values.next().expect("missing destination start");
                let src_start = values.next().expect("missing source start");
                let range_length = values.next().expect("missing range length");
                (
                    src_start,
                    AlmanacMap {
                        range_length,
                        dest_start,
                    },
                )
            })
            .collect();
        let mapping = AlmanacMapping {
            dest_category: dest_category.to_owned(),
            maps,
        };
        almanac_mappings.insert(src_category, mapping);
    }
    let nearest_location = seeds
        .chunks_exact(2)
        .flat_map(|chunk| match *chunk {
            [start, length] => start..(start + length),
            _ => unreachable!(),
        })
        .map(|seed| {
            let mut number = seed;
            let mut category = "seed";
            loop {
                let mapping = &almanac_mappings[&category];
                let last_map = mapping.maps.range(..=number).next_back();
                if let Some((&src_start, AlmanacMap { dest_start, .. })) =
                    last_map.filter(|(&src_start, AlmanacMap { range_length, .. })| {
                        number <= src_start + (range_length - 1)
                    })
                {
                    let diff = dest_start.abs_diff(src_start);
                    if *dest_start > src_start {
                        number += diff;
                    } else {
                        number -= diff;
                    }
                }
                category = &mapping.dest_category;
                if category == "location" {
                    return number;
                }
            }
        })
        .min()
        .expect("seeds list was empty");
    println!("The lowest location number is {nearest_location}!");
}
