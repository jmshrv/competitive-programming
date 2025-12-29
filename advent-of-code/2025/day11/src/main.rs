use std::{
    collections::{HashMap, VecDeque},
    io,
};

fn paths(routes: &HashMap<String, Vec<String>>) -> u64 {
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

    let part1_answer = paths(&input);
    println!("{part1_answer}");
}
