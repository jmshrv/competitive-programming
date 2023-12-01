use std::io::{self, Lines, StdinLock};

use itertools::Itertools;

fn parse_inventory(inventory: Lines<StdinLock>) -> Vec<Vec<usize>> {
    let mut inventory_list = Vec::new();

    let mut current_elf: Vec<usize> = Vec::new();

    for line_res in inventory {
        let line = line_res.unwrap();

        if line.is_empty() {
            inventory_list.push(current_elf);
            current_elf = Vec::new();
            continue;
        }

        let line_parsed: usize = line.parse().unwrap();
        current_elf.push(line_parsed);
    }

    if !current_elf.is_empty() {
        inventory_list.push(current_elf);
    }

    inventory_list
}

fn main() {
    let input = io::stdin().lines();
    let inventories = parse_inventory(input);

    let top_three: Vec<usize> = inventories
        .iter()
        .map(|x| x.iter().sum::<usize>())
        .sorted_unstable()
        .rev()
        .take(3)
        .collect();

    println!("{}", top_three.iter().max().unwrap());
    println!("{}", top_three.iter().sum::<usize>())
}
