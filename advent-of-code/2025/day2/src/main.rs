use std::{
    io::{self},
    ops::RangeInclusive,
};

fn is_mirror(id: usize) -> bool {
    let digits = id.ilog10() + 1;

    if digits % 2 == 1 {
        return false;
    }

    let divisor = 10_usize.pow(digits / 2);

    let left = id / divisor;
    let right = id % divisor;

    left == right
}

fn mirror_ids(ids: RangeInclusive<usize>) -> impl Iterator<Item = usize> {
    ids.filter(|id| is_mirror(*id))
}

fn main() {
    let input = io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|range_str| range_str.split_once('-').unwrap())
        .map(|(start_str, end_str)| {
            start_str.parse::<usize>().unwrap()..=end_str.parse::<usize>().unwrap()
        })
        .collect::<Vec<_>>();

    let part1_answer: usize = input.iter().flat_map(|ids| mirror_ids(ids.clone())).sum();

    println!("{part1_answer}");
}
