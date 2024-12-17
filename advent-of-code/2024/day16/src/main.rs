use std::io;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Start,
    End,
    Empty,
    Wall,
}

fn main() {
    let map = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|row| {
            row.chars()
                .map(|char| match char {
                    'S' => Tile::Start,
                    'E' => Tile::End,
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    _ => panic!("Invalid tile {char}!"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, tile)| {
                if *tile == Tile::Start {
                    Some((y, x))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let end = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, tile)| {
                if *tile == Tile::End {
                    Some((y, x))
                } else {
                    None
                }
            })
        })
        .unwrap();
}
