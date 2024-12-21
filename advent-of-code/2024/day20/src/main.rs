use std::{
    collections::{HashMap, VecDeque},
    io,
};

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Start,
    End,
}

fn parse_line(line: &str) -> Vec<Tile> {
    line.chars()
        .map(|tile| match tile {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => panic!("Invalid tile {tile}!"),
        })
        .collect()
}

fn bfs(map: &[Vec<Tile>]) -> HashMap<(usize, usize), u64> {
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, tile)| (*tile == Tile::Start).then_some((y, x)))
        })
        .unwrap();

    let mut queue = VecDeque::from([(start, 0)]);
    let mut visited = HashMap::new();

    while let Some((position, distance)) = queue.pop_front() {
        if visited.contains_key(&position) {
            continue;
        }

        visited.insert(position, distance);

        let neighbours = [
            ((position.0 - 1, position.1), distance + 1),
            ((position.0 + 1, position.1), distance + 1),
            ((position.0, position.1 - 1), distance + 1),
            ((position.0, position.1 + 1), distance + 1),
        ]
        .into_iter()
        .filter_map(|neighbour| {
            map.get(neighbour.0 .0)?
                .get(neighbour.0 .1)
                .map(|tile| (neighbour, tile))
        })
        .filter_map(|(neighbour, tile)| (*tile != Tile::Wall).then_some(neighbour));

        queue.extend(neighbours);
    }

    visited
}

fn cheat_savings(
    distances: &HashMap<(usize, usize), u64>,
    max_cheat: u64,
    min_savings: u64,
) -> usize {
    let mut count = 0;

    for (i, (p1, p1_dist)) in distances.iter().enumerate() {
        for (p2, p2_dist) in distances.iter().skip(i + 1) {
            let distance = (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as u64;

            if distance <= max_cheat
                && p1_dist
                    .abs_diff(*p2_dist)
                    .checked_sub(distance)
                    .is_some_and(|saved| saved >= min_savings)
            {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| parse_line(&line))
        .collect::<Vec<_>>();

    let path = bfs(&input);

    let part_one = cheat_savings(&path, 2, 100);

    println!("{part_one}");

    let part_two = cheat_savings(&path, 20, 100);

    println!("{part_two}");
}
