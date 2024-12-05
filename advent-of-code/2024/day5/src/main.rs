use std::io;

fn is_update_valid(rules: &[(u32, u32)], update: &[u32]) -> bool {
    rules
        .iter()
        .filter_map(|(from, to)| {
            match (
                update.iter().position(|&page| page == *from),
                update.iter().position(|&page| page == *to),
            ) {
                (Some(from_index), Some(to_index)) => Some((from_index, to_index)),
                _ => None,
            }
        })
        .all(|(from_index, to_index)| from_index < to_index)
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (rules_input, updates_input) = input.split_once("\n\n").unwrap();

    let rules = rules_input
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();

    let updates = updates_input.lines().map(|line| {
        line.split(',')
            .map(|update_str| update_str.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    });

    let part_one: u32 = updates
        .filter(|update| is_update_valid(&rules, &update))
        .map(|valid_update| valid_update[valid_update.len() / 2])
        .sum();

    println!("{part_one}");
}
