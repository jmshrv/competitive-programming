use std::{
    collections::{HashMap, HashSet},
    io,
    str::Chars,
};

use itertools::Itertools;

fn duplicates(backpacks: &[String]) -> HashSet<char> {
    let mut unique_characters: HashMap<char, usize> = HashMap::new();

    for backpack in backpacks {
        for char in backpack.chars().unique() {
            let count = *unique_characters.get(&char).unwrap_or(&0);
            unique_characters.insert(char, count + 1);
        }
    }

    unique_characters
        .iter()
        .filter(|item| *item.1 == backpacks.len())
        .map(|kv| *kv.0)
        .collect()
}

trait Priority {
    fn priority(self) -> u8;
}

impl Priority for char {
    fn priority(self) -> u8 {
        // In a proper piece of software you'd have an assert here to ensure
        // that the character is ascii, but that would slow us down ;)
        if self.is_ascii_lowercase() {
            return self as u8 - 96;
        }

        self as u8 - 38
    }
}

fn main() {
    let input: Vec<_> = io::stdin().lines().map(|line| line.unwrap()).collect();

    let priority_sum_part_1: u32 = input
        .iter()
        .map(|line| {
            let halves = line.split_at(line.chars().count() / 2);
            duplicates(&[halves.0.to_string(), halves.1.to_string()])
                .iter()
                .map(|y| y.priority() as u32)
                .sum::<u32>()
        })
        .sum();

    let priority_sum_part_2: u32 = input
        .chunks_exact(3)
        .map(|chunk| {
            duplicates(chunk)
                .iter()
                .map(|y| y.priority() as u32)
                .sum::<u32>()
        })
        .sum();

    println!("{}", priority_sum_part_1);
    println!("{}", priority_sum_part_2);
}
