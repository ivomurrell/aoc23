use std::{
    collections::{hash_map::Entry, HashMap},
    fs::read_to_string,
};

fn hash(label: &str) -> u8 {
    label.bytes().fold(0, |current_value, ch| {
        ((current_value as u16 + ch as u16) * 17) as u8
    })
}

fn main() {
    let input = read_to_string("../input.txt").expect("failed to open input file");
    let mut boxes: HashMap<u8, Vec<(&str, u8)>> = HashMap::new();
    for step in input.trim_end().split(',') {
        match step.split_once('=') {
            Some((label, length)) => {
                let lens_box = boxes.entry(hash(label)).or_default();
                let length = length.parse().expect("failed to parse focal length");
                match lens_box
                    .iter_mut()
                    .find(|&&mut (lens_lab, _)| lens_lab == label)
                {
                    Some((_, lens_length)) => {
                        *lens_length = length;
                    }
                    None => lens_box.push((label, length)),
                }
            }
            _ => {
                let label = &step[..step.len() - 1];
                if let Entry::Occupied(mut box_entry) = boxes.entry(hash(label)) {
                    let lens_box = box_entry.get_mut();
                    if let Some(i) = lens_box.iter().position(|&(lens_lab, _)| lens_lab == label) {
                        lens_box.remove(i);
                    }
                }
            }
        }
    }
    let power_sum: usize = boxes
        .into_iter()
        .map(|(box_num, lens_box)| {
            lens_box
                .into_iter()
                .enumerate()
                .map(|(slot, (_, focal_length))| {
                    (box_num as usize + 1) * (slot + 1) * focal_length as usize
                })
                .sum::<usize>()
        })
        .sum();
    println!("The focusing power of all the lenses is {power_sum}!");
}
