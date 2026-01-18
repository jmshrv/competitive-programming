use std::{collections::HashMap, io};

use cached::proc_macro::cached;

#[cached(
    key = "(String, bool, bool)",
    convert = r#"{ (node.to_string(), seen_dac, seen_fft) }"#
)]
fn paths(routes: &HashMap<String, Vec<String>>, node: &str, seen_dac: bool, seen_fft: bool) -> u64 {
    let mut valid_routes = 0;

    if node == "out" {
        return (seen_dac && seen_fft) as u64;
    }

    for child in &routes[node] {
        let res = paths(
            routes,
            child,
            seen_dac || node == "dac",
            seen_fft || node == "fft",
        );

        valid_routes += res;
    }

    valid_routes
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

    let part1_answer = paths(&input, "you", true, true);
    println!("{part1_answer}");

    let part2_answer = paths(&input, "svr", false, false);
    println!("{part2_answer}");
}
