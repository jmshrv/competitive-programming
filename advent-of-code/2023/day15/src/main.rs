use std::{
    collections::{BTreeMap, HashMap},
    io,
};

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
        let (identifier, value) = step.split_once(['-', '=']).expect("Failed to split step!");

        if step.ends_with('-') {
            if let Some(lens_box) = map.get_mut(&hash(*step)) {
                if let Some(index) = lens_box
                    .iter()
                    .position(|(box_id, _)| *box_id == identifier)
                {
                    lens_box.remove(index);
                }
            }
        }
    }

    todo!()
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .next()
        .expect("No line!");

    let split_input = input.split(',').collect::<Vec<_>>();

    let part1_answer: u64 = split_input.iter().map(|step| hash(step)).sum();

    println!("{part1_answer}");

    let mut part2_answer = part2(&split_input);
}
