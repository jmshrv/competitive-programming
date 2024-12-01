use std::io;

fn main() {
    let (mut left_list, mut right_list): (Vec<_>, Vec<_>) = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(left, right)| (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap()))
        .unzip();

    left_list.sort_unstable();
    right_list.sort_unstable();

    let part_one: i64 = left_list
        .iter()
        .zip(right_list.clone())
        .map(|(left, right)| i64::abs(left.max(&right) - left.min(&right)))
        .sum();

    println!("{part_one}");

    let part_two: i64 = left_list
        .iter()
        .map(|left| *left * right_list.iter().filter(|right| left == *right).count() as i64)
        .sum();

    println!("{part_two}");
}
