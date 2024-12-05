use std::io;

fn next_rule_to_fix(rules: &[(u32, u32)], update: &[u32]) -> Option<(usize, usize)> {
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
        .find(|(from_index, to_index)| from_index > to_index)
}

fn is_update_valid(rules: &[(u32, u32)], update: &[u32]) -> bool {
    next_rule_to_fix(rules, update).is_none()
}

fn fix_invalid_update(rules: &[(u32, u32)], update: &mut [u32]) {
    let relevant_rules = rules
        .iter()
        .filter(|(from, to)| update.contains(from) && update.contains(to))
        .map(|&(from, to)| (from, to))
        .collect::<Vec<_>>();

    while let Some((from_index, to_index)) = next_rule_to_fix(&relevant_rules, &update) {
        update.swap(from_index, to_index);
    }
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (rules_input, updates_input) = input.split_once("\n\n").unwrap();

    let rules = rules_input
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();

    let mut updates = updates_input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|update_str| update_str.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part_one: u32 = updates
        .iter()
        .filter(|update| is_update_valid(&rules, update))
        .map(|valid_update| valid_update[valid_update.len() / 2])
        .sum();

    println!("{part_one}");

    let part_two: u32 = updates
        .iter_mut()
        .filter(|update| !is_update_valid(&rules, update))
        .map(|invalid_update| {
            fix_invalid_update(&rules, invalid_update);
            invalid_update
        })
        .map(|valid_update| valid_update[valid_update.len() / 2])
        .sum();

    println!("{part_two}");
}
