use std::io;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle};
use itertools::Itertools;
use rayon::prelude::*;

struct Tile {
    x: usize,
    y: usize,
}

impl Tile {
    fn area(&self, other: &Self) -> usize {
        // We add 1 to consider the width of the tile itself
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }

    fn points(&self, other: &Self) -> impl Iterator<Item = Tile> {
        (self.x.min(other.x)..=self.x.max(other.x))
            .cartesian_product(self.y.min(other.y)..=self.y.max(other.y))
            .map(|(x, y)| Tile { x, y })
    }
}

fn debug_print(map: &[Vec<u8>]) {
    for row in map {
        for cell in row {
            print!("{}", *cell as char);
        }
        println!();
    }
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let (x_str, y_str) = line.split_once(',').unwrap();

            Tile {
                x: x_str.parse().unwrap(),
                y: y_str.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let part1_answer = input
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.area(b))
        .max()
        .unwrap();
    println!("{part1_answer}");

    let ymax = input.iter().max_by(|&a, &b| a.y.cmp(&b.y)).unwrap().y;
    let xmax = input.iter().max_by(|&a, &b| a.x.cmp(&b.x)).unwrap().x;

    println!("Generating map...");
    let mut map = vec![vec![b'.'; xmax + 1]; ymax + 1];
    println!("Map generated!");

    println!("Drawing lines...");

    for (a, b) in input.iter().chain([&input[0]]).tuple_windows() {
        for point in a.points(b) {
            map[point.y][point.x] = b'X';
        }

        map[a.y][a.x] = b'#';
        map[b.y][b.x] = b'#';
    }

    println!("Filling in map...");

    for row in &mut map {
        let start_res = row.iter().position(|&cell| cell != b'.');
        let end_res = row.iter().rev().position(|&cell| cell != b'.');

        if let (Some(start), Some(end)) = (start_res, end_res) {
            let real_start = start + 1; // Don't draw over the wall
            let real_end = row.len() - end - 1; // We rev'd the row, so have to reverse here

            row[real_start..real_end].fill(b'X');
        }
    }

    println!("Calculating part 2 answer...");

    let part2_answer = input
        .iter()
        .tuple_combinations()
        .par_bridge()
        .progress_count(input.iter().tuple_combinations::<(_, _)>().count() as u64)
        .filter(|(a, b)| a.points(b).all(|point| map[point.y][point.x] != b'.'))
        .map(|(a, b)| a.area(b))
        .max()
        .unwrap();

    println!("{part2_answer}");
}
