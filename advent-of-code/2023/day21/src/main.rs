use std::{
    collections::{HashSet, VecDeque},
    io,
};

fn adjacent(
    input: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    steps: usize,
) -> Vec<(usize, usize, usize)> {
    let mut adj = vec![];

    if x != 0 {
        if input[y][x - 1] != '#' {
            adj.push((x - 1, y, steps + 1));
        }
    }

    if input[y][x + 1] != '#' {
        adj.push((x + 1, y, steps + 1));
    }

    if y != 0 {
        if input[y - 1][x] != '#' {
            adj.push((x, y - 1, steps + 1));
        }
    }

    if input[y + 1][x] != '#' {
        adj.push((x, y + 1, steps + 1));
    }

    adj
}

fn traverse(input: &Vec<Vec<char>>, max_steps: usize) -> usize {
    let start_y = input
        .iter()
        .position(|line| line.contains(&'S'))
        .expect("Failed to find start y!");

    let start_x = input[start_y]
        .iter()
        .position(|char| char == &'S')
        .expect("Failed to find start x!");

    let mut queue = VecDeque::from([(start_x, start_y, 0)]);
    let mut explored = HashSet::from([(start_x, start_y, 0)]);

    while let Some((x, y, steps)) = queue.pop_front() {
        if steps == max_steps {
            return queue.len() + 1;
        }

        for edge in adjacent(input, x, y, steps) {
            if explored.insert(edge) {
                queue.push_back(edge);
            }
        }
    }

    todo!()
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let part1_answer = traverse(&input, 64);

    println!("{part1_answer}");
}
