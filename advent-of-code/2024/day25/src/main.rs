use std::io;

use itertools::Itertools;

fn heights(schematic: &str) -> [u8; 5] {
    schematic
        .lines()
        .skip(1)
        .take(5)
        .map(|line| line.chars().map(|char| if char == '#' { 1 } else { 0 }))
        .fold([0, 0, 0, 0, 0], |acc, line| {
            acc.iter()
                .zip(line)
                .map(|(acc_row, line_row)| acc_row + line_row)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (locks, keys): (Vec<_>, Vec<_>) = input.split("\n\n").partition(|schematic| {
        schematic
            .lines()
            .next()
            .unwrap()
            .chars()
            .all(|char| char == '#')
            && schematic
                .lines()
                .last()
                .unwrap()
                .chars()
                .all(|char| char == '.')
    });

    let lock_heights = locks.iter().map(|lock| heights(lock)).collect::<Vec<_>>();

    let key_heights = keys.iter().map(|key| heights(key)).collect::<Vec<_>>();

    let answer = lock_heights
        .iter()
        .cartesian_product(key_heights)
        .filter(|(lock, key)| {
            lock.iter()
                .zip(key)
                .all(|(lock_height, key_height)| lock_height + key_height <= 5)
        })
        .count();

    println!("{answer}");
}
