#![warn(clippy::pedantic)]

use std::{
    cmp::{max, min},
    io,
    ops::Range,
};

fn range_from_string(input: &str) -> Range<u8> {
    let (start_str, end_str) = input.split_once('-').unwrap();

    let start = start_str.parse().unwrap();
    let end = end_str.parse().unwrap();

    start..end
}

// Stolen from:
// https://www.reddit.com/r/rust/comments/aynxgl/is_there_a_rangetools_crate/
fn intersect(a: &Range<u8>, b: &Range<u8>) -> Range<u8> {
    max(a.start, b.start)..min(a.end, b.end)
}

fn any_intersect(a: &Range<u8>, b: &Range<u8>) -> bool {
    !(a.end < b.start || a.start > b.end)
}

fn main() {
    let input: Vec<_> = io::stdin()
        .lines()
        .map(|line_res| {
            let line = line_res.unwrap();
            let line_split = line.split_once(',').unwrap();
            (
                range_from_string(line_split.0),
                range_from_string(line_split.1),
            )
        })
        .collect();

    let answer_1 = input
        .iter()
        .filter(|pair| {
            let intersect = intersect(&pair.0, &pair.1);
            intersect == pair.0 || intersect == pair.1
        })
        .count();

    let answer_2 = input
        .iter()
        .filter(|pair| any_intersect(&pair.0, &pair.1))
        .count();

    println!("{}", answer_1);
    println!("{}", answer_2);
}
