use std::{collections::HashMap, io};

fn valid_designs<'a>(
    design: &'a str,
    inventory: &[&str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(cached_answer) = cache.get(design) {
        return *cached_answer;
    }

    if design.is_empty() {
        return 1;
    }

    let answer = inventory
        .iter()
        .filter(|inventory_design| design.starts_with(**inventory_design))
        .map(|valid_design| {
            valid_designs(design.strip_prefix(valid_design).unwrap(), inventory, cache)
        })
        .sum();

    cache.insert(design, answer);

    answer
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (inventory_str, designs_str) = input.split_once("\n\n").unwrap();

    let inventory = inventory_str.split(", ").collect::<Vec<_>>();
    let designs = designs_str.lines().collect::<Vec<_>>();

    let part_one = designs
        .iter()
        .filter(|design| valid_designs(design, &inventory, &mut HashMap::new()) != 0)
        .count();

    println!("{part_one}");

    let part_two: usize = designs
        .iter()
        .map(|design| valid_designs(design, &inventory, &mut HashMap::new()))
        .sum();

    println!("{part_two}");
}
