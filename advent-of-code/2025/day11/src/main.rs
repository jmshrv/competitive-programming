use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
};

fn part1_paths(routes: &HashMap<String, Vec<String>>) -> u64 {
    let mut queue = VecDeque::from(routes["you"].clone());

    let mut path_count = 0;

    // Just a bfs without a seen set lol
    // Good thing the inputs never loop on themselves

    while let Some(path) = queue.pop_front() {
        if path == "out" {
            path_count += 1;
            continue;
        }

        for next_path in &routes[&path] {
            queue.push_back(next_path.clone());
        }
    }

    path_count
}

fn part2_paths(routes: &HashMap<String, Vec<String>>) -> u64 {
    let mut queue = routes["svr"]
        .iter()
        .map(|server| (server.clone(), false, false))
        .collect::<VecDeque<_>>();

    let mut path_count = 0;

    while let Some((path, seen_dac, seen_fft)) = queue.pop_front() {
        if seen_dac && seen_fft && path == "out" {
            path_count += 1;
        }

        // This is in a separate check since we still need to continue on "out" when we haven't seen
        // a DAC and an FFT
        if path == "out" {
            continue;
        }

        for next_path in &routes[&path] {
            queue.push_back((
                next_path.clone(),
                seen_dac || path == "dac",
                seen_fft || path == "fft",
            ));
        }
    }

    path_count
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let (node_str, edges_str) = line.split_once(':').unwrap();

            let edges = edges_str
                .split_ascii_whitespace()
                .map(str::to_string)
                .collect::<Vec<_>>();

            (node_str.to_string(), edges)
        })
        .collect::<HashMap<_, _>>();

    let part1_answer = part1_paths(&input);
    println!("{part1_answer}");

    let part2_answer = part2_paths(&input);
    println!("{part2_answer}");
}
