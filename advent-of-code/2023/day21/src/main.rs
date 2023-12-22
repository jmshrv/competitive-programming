use std::{
    collections::{HashMap, HashSet, VecDeque},
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

    if x + 1 < input.first().unwrap().len() {
        if input[y][x + 1] != '#' {
            adj.push((x + 1, y, steps + 1));
        }
    }

    if y != 0 {
        if input[y - 1][x] != '#' {
            adj.push((x, y - 1, steps + 1));
        }
    }

    if y + 1 < input.len() {
        if input[y + 1][x] != '#' {
            adj.push((x, y + 1, steps + 1));
        }
    }

    adj
}

fn traverse(input: &Vec<Vec<char>>, max_steps: usize) -> (usize, usize) {
    let start_y = input
        .iter()
        .position(|line| line.contains(&'S'))
        .expect("Failed to find start y!");

    let start_x = input[start_y]
        .iter()
        .position(|char| char == &'S')
        .expect("Failed to find start x!");

    let mut queue = VecDeque::from([(start_x, start_y, 0)]);
    // let mut explored = HashMap::from([((start_x, start_y), 0)]);
    let mut explored = HashMap::new();

    while let Some((x, y, steps)) = queue.pop_front() {
        if explored.contains_key(&(x, y)) {
            continue;
        }

        explored.insert((x, y), steps);

        for edge in adjacent(input, x, y, steps) {
            if !explored.contains_key(&(edge.0, edge.1)) {
                queue.push_back(edge);
            }
        }
    }

    let part1_answer = explored
        .values()
        .filter(|distance| **distance <= 64 && **distance % 2 == 0)
        .count();

    let even_corners = explored
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = explored
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    let n = 202300;

    let even = n * n;
    let odd = (n + 1) * (n + 1);

    let part2_answer = odd * explored.values().filter(|v| **v % 2 == 1).count()
        + even * explored.values().filter(|v| **v % 2 == 0).count()
        - ((n + 1) * odd_corners)
        + (n * even_corners);

    (part1_answer, part2_answer)
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (part1_answer, part2_answer) = traverse(&input, 64);

    println!("{part1_answer}");
    println!("{part2_answer}");
}
