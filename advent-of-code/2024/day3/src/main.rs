use std::{io, usize};

use regex::Regex;

fn is_enabled(index: usize, dos: &[usize], donts: &[usize]) -> bool {
    let last_do = dos
        .iter()
        .take_while(|do_index| index > **do_index)
        .last()
        .unwrap_or(&usize::MAX);
    let last_dont = donts
        .iter()
        .take_while(|dont_index| index > **dont_index)
        .last()
        .unwrap_or(&0);

    last_do > last_dont
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Stored in (index, result) form
    let multiplications = regex
        .captures_iter(&input)
        .map(|captures| {
            (
                captures.get(0).unwrap().start(),
                captures[1].parse::<u32>().unwrap(),
                captures[2].parse::<u32>().unwrap(),
            )
        })
        .map(|(index, a, b)| (index, a * b))
        .collect::<Vec<_>>();

    let part_one: u32 = multiplications.iter().map(|(_, result)| result).sum();

    println!("{part_one}");

    let dos = input
        .match_indices("do()")
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    let donts = input
        .match_indices("don't()")
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    let part_two: u32 = multiplications
        .iter()
        .filter(|(index, _)| is_enabled(*index, &dos, &donts))
        .map(|(_, result)| result)
        .sum();

    println!("{part_two}");
}
