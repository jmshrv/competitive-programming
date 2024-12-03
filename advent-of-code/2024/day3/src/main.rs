use std::{collections::HashSet, io};

use regex::Regex;

fn is_enabled(index: usize, dos: &HashSet<usize>, donts: &HashSet<usize>) -> bool {
    let mut result = true;

    for i in 0..index {
        if dos.contains(&i) {
            result = true;
        } else if donts.contains(&i) {
            result = false;
        }
    }

    result
}

fn main() {
    let input: String = io::stdin().lines().flatten().collect();

    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

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
        .collect::<HashSet<_>>();

    let donts = input
        .match_indices("don't()")
        .map(|(index, _)| index)
        .collect::<HashSet<_>>();

    let part_two: u32 = multiplications
        .iter()
        .filter(|(index, _)| is_enabled(*index, &dos, &donts))
        .map(|(_, result)| result)
        .sum();

    println!("{part_two}");
}
