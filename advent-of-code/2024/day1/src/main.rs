use std::{collections::HashMap, io};

fn main() {
    let (mut left_list, mut right_list): (Vec<_>, Vec<_>) = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(left, right)| (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap()))
        .unzip();

    left_list.sort_unstable();
    right_list.sort_unstable();

    let part_one: i64 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    println!("{part_one}");

    let counts = right_list
        .iter()
        .fold(HashMap::<i64, i64>::new(), |mut acc, right| {
            *acc.entry(*right).or_insert(0) += 1;
            acc
        });

    let part_two: i64 = left_list
        .iter()
        .map(|left| *left * counts.get(left).unwrap_or(&0))
        .sum();

    println!("{part_two}");
}
