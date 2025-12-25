use std::io;

use itertools::Itertools;

struct Tile {
    x: u64,
    y: u64,
}

impl Tile {
    fn area(&self, other: &Self) -> u64 {
        // We add 1 to consider the width of the tile itself
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn areas(map: &[Tile]) -> impl Iterator<Item = u64> {
    map.iter().tuple_combinations().map(|(a, b)| a.area(b))
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

    let part1_answer = areas(&input).max().unwrap();
    println!("{part1_answer}");
}
