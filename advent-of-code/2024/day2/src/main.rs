use std::{cmp::Ordering, io};

use itertools::Itertools;

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .iter()
        .map(|line| {
            line.split(' ')
                .map(|entry| entry.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part_one = input
        .iter()
        .filter(|line| {
            line.iter()
                .tuple_windows::<(_, _)>()
                .map(|(a, b)| a.cmp(b))
                .filter(|ord| *ord != Ordering::Equal)
                .all_equal()
        })
        .filter(|line| {
            line.iter()
                .tuple_windows::<(_, _)>()
                .map(|(a, b)| (a - b).abs())
                .all(|difference| difference >= 1 && difference <= 3)
        })
        .count();

    println!("{part_one}");
}
