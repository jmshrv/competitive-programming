use std::{
    collections::{HashSet, VecDeque},
    io,
};

fn bfs(map: &HashSet<(u8, u8)>) -> u16 {
    let max_size = 70;
    let end = (max_size, max_size);

    let mut queue = VecDeque::from([((0, 0), 0)]);
    let mut seen = HashSet::new();

    while let Some((position, steps)) = queue.pop_front() {
        if !seen.insert(position) {
            continue;
        }

        if position == end {
            return steps;
        }

        let neighbours = [
            ((position.0, position.1 - 1), steps + 1),
            ((position.0, position.1 + 1), steps + 1),
            ((position.0 + 1, position.1), steps + 1),
            ((position.0 - 1, position.1), steps + 1),
        ]
        .into_iter()
        .filter(|(neighbour, _)| neighbour.0 <= max_size && neighbour.1 <= max_size)
        .filter(|(neighbour, _)| !map.contains(neighbour));

        queue.extend(neighbours);
    }

    panic!("Failed to find end!")
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u8>().unwrap(), y.parse::<u8>().unwrap())
        })
        .collect::<Vec<_>>();

    let map = input.clone().into_iter().take(1024).collect();

    let part_one = bfs(&map);

    println!("{part_one}");
}
