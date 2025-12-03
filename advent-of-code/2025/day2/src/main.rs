use std::{
    io::{self},
    ops::RangeInclusive,
};

use itertools::Itertools;

fn digit_slices(id: usize, slice_count: usize) -> Vec<usize> {
    // yeah I can't be bothered to do the maths here

    let id_str = id.to_string();

    id_str
        .chars()
        .chunks(slice_count)
        .into_iter()
        .map(|chunk| chunk.collect::<String>().parse().unwrap())
        .collect()
}

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

fn part2_invalid_ids(ids: RangeInclusive<usize>) -> impl Iterator<Item = usize> {
    ids.filter(|id| {
        let digits = id.ilog10() + 1;
        (1..=digits).any(|chunk_count| {
            if digits % chunk_count != 0 {
                return false;
            }

            let slices = digit_slices(*id, chunk_count as usize);
            let slice_counts = slices.iter().counts();

            if slice_counts.len() != 1 {
                return false;
            }

            *slice_counts.values().next().unwrap() >= 2
        })
    })
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

    let part2_answer: usize = input
        .iter()
        .flat_map(|ids| part2_invalid_ids(ids.clone()))
        .sum();

    println!("{part2_answer}");
}
