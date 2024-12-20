use std::{
    collections::{HashSet, VecDeque},
    io,
};

fn bfs(map: &HashSet<&(u8, u8)>) -> Option<u16> {
    let max_size = 70;
    let end = (max_size, max_size);

    let mut queue = VecDeque::from([((0, 0), 0)]);
    let mut seen = HashSet::new();

    while let Some((position, steps)) = queue.pop_front() {
        if !seen.insert(position) {
            continue;
        }

        if position == end {
            return Some(steps);
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

    None
}

fn main() {
    let initial_num_bytes = 1024;

    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u8>().unwrap(), y.parse::<u8>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut map = input.iter().take(initial_num_bytes).collect();

    let part_one = bfs(&map).unwrap();

    println!("{part_one}");

    for byte in &input[initial_num_bytes..] {
        map.insert(byte);

        if bfs(&map).is_none() {
            println!("{},{}", byte.0, byte.1);
            return;
        }
    }

    panic!("Failed to find part 2!")
}
