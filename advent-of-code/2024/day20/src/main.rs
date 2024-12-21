use std::{
    collections::{HashSet, VecDeque},
    io,
};

use itertools::Itertools;

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

fn bfs(map: &[Vec<Tile>]) -> Vec<(usize, usize)> {
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, tile)| (*tile == Tile::Start).then_some((y, x)))
        })
        .unwrap();

    let mut queue = VecDeque::from([(start, vec![start])]);
    let mut visited = HashSet::new();

    while let Some((position, mut path)) = queue.pop_front() {
        if !visited.insert(position) {
            continue;
        }

        if map[position.0][position.1] == Tile::End {
            return path;
        }

        path.push(position);

        let neighbours = [
            ((position.0 - 1, position.1), path.clone()),
            ((position.0 + 1, position.1), path.clone()),
            ((position.0, position.1 - 1), path.clone()),
            ((position.0, position.1 + 1), path.clone()),
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

    panic!("No path from start to end!");
}

fn cheat_savings(
    map: &HashSet<(usize, usize)>,
    path: &[(usize, usize)],
    max_cheat: usize,
    min_savings: usize,
) -> usize {
    // path.iter()
    //     .enumerate()
    //     .cartesian_product(path.iter().enumerate().skip(1))
    //     .filter(|((_, p1), (_, p2))| p1 != p2)
    //     .filter(|((_, p1), (_, p2))| p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1) == max_cheat)
    //     .map(|((dist1, _), (dist2, _))| dist1.abs_diff(dist2) - max_cheat)
    //     .filter(|savings| *savings >= min_savings)
    //     .count()

    let mut count = 0;

    for (i, p1) in map.iter().enumerate() {
        for p2 in map.iter().skip(i + 1) {
            let distance = p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1);

            if distance <= max_cheat {
                let p1_dist = path.iter().position(|pos| pos == p1);
                let p2_dist = path.iter().position(|pos| pos == p2);

                if let (Some(p1_dist), Some(p2_dist)) = (p1_dist, p2_dist) {
                    if p1_dist
                        .abs_diff(p2_dist)
                        .checked_sub(distance)
                        .is_some_and(|saved| saved >= min_savings)
                    {
                        count += 1;
                    }
                }
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

    let map = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, tile)| (*tile != Tile::Wall).then_some((y, x)))
        })
        .collect();

    let part_one = cheat_savings(&map, &path, 2, 64);

    println!("{part_one}");
}
