use std::{collections::HashMap, io};

// why of course we need to optimally use string refs everywhere
fn paths<'a>(
    routes: &'a HashMap<String, Vec<String>>,
    cache: &mut HashMap<(&'a str, bool, bool), u64>,
    node: &'a str,
    seen_dac: bool,
    seen_fft: bool,
) -> u64 {
    if let Some(&cached) = cache.get(&(node, seen_dac, seen_fft)) {
        return cached;
    }

    let mut valid_routes = 0;

    if node == "out" {
        return (seen_dac && seen_fft) as u64;
    }

    for child in &routes[node] {
        let res = paths(
            routes,
            cache,
            child,
            seen_dac || node == "dac",
            seen_fft || node == "fft",
        );

        valid_routes += res;
        cache.insert((child, seen_dac, seen_fft), res);
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

    let part1_answer = paths(&input, &mut HashMap::new(), "you", true, true);
    println!("{part1_answer}");

    let part2_answer = paths(&input, &mut HashMap::new(), "svr", false, false);
    println!("{part2_answer}");
}
