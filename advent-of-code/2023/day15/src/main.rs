use std::{collections::HashMap, io};

fn hash(input: &str) -> u64 {
    let mut res = 0;

    for char in input.chars() {
        res += char as u64;
        res *= 17;
        res %= 256;
    }

    res
}

fn part2(input: &Vec<&str>) -> u64 {
    let mut map: HashMap<u64, Vec<(&str, u64)>> = HashMap::new();

    for step in input {
        let (identifier, focal_length_str) =
            step.split_once(['-', '=']).expect("Failed to split step!");
        let hash_key = hash(identifier);

        if step.ends_with('-') {
            if let Some(lens_box) = map.get_mut(&hash_key) {
                if let Some(index) = lens_box
                    .iter()
                    .position(|(box_id, _)| *box_id == identifier)
                {
                    lens_box.remove(index);
                }
            }
        } else {
            let focal_length = focal_length_str
                .parse::<u64>()
                .expect("Failed to parse focal length!");

            map.entry(hash_key)
                .and_modify(|lens_box| {
                    if let Some(lens) = lens_box.iter_mut().find(|index| index.0 == identifier) {
                        lens.1 = focal_length;
                    } else {
                        lens_box.push((identifier, focal_length));
                    }
                })
                .or_insert(vec![(identifier, focal_length)]);
        }
    }

    map.iter()
        .map(|(box_number, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(|(index, (_, focal_length))| {
                    (box_number + 1) * (index as u64 + 1) * focal_length
                })
                .sum::<u64>()
        })
        .sum()
}

fn main() {
    let input = io::stdin()
        .lines()
        .find_map(|res| res.ok())
        .expect("No line!");

    let split_input = input.split(',').collect::<Vec<_>>();

    let part1_answer: u64 = split_input.iter().map(|step| hash(step)).sum();
    println!("{part1_answer}");

    let part2_answer = part2(&split_input);
    println!("{part2_answer}");
}
