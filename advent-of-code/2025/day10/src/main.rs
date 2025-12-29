use std::{
    collections::{HashSet, VecDeque},
    io,
};

use good_lp::variables;
use lazy_regex::regex;

#[derive(Debug)]
struct Machine {
    indicator_lights: Vec<bool>,
    switches: Vec<Vec<usize>>,
    joltages: Vec<u64>,
}

struct SearchState<T> {
    depth: u64,
    state: Vec<T>,
}

fn parse_line(line: &str) -> Machine {
    let regex = regex!(r"(\[(?:\.|#)*\])(?: )(\(.*\))(?: )(\{.*\})");

    let (_, [indicator_lights_str, switches_str, joltages_str]) =
        regex.captures(line).unwrap().extract::<3>();

    let indicator_lights = indicator_lights_str
        .chars()
        .filter_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            '[' | ']' => None,
            _ => panic!("Invalid indicator light {c}"), // want to actually notice invalid
        })
        .collect();

    let switches = switches_str
        .split_ascii_whitespace()
        .map(|switch| {
            switch
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(',')
                .map(|idx| idx.parse().unwrap())
                .collect()
        })
        .collect();

    let joltages = joltages_str
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(',')
        .map(|joltage| joltage.parse().unwrap())
        .collect();

    Machine {
        indicator_lights,
        switches,
        joltages,
    }
}

fn light_presses(machine: &Machine) -> u64 {
    let mut queue = VecDeque::from([SearchState {
        depth: 0,
        state: vec![false; machine.indicator_lights.len()],
    }]);

    let mut seen = HashSet::new();

    while let Some(state) = queue.pop_front() {
        if state.state == machine.indicator_lights {
            return state.depth;
        }

        if !seen.insert(state.state.clone()) {
            continue;
        }

        for switch in &machine.switches {
            let mut new_lights = state.state.clone();

            for &idx in switch {
                new_lights[idx] = !new_lights[idx];
            }

            queue.push_back(SearchState {
                depth: state.depth + 1,
                state: new_lights,
            });
        }
    }

    unreachable!()
}

fn joltage_presses(machine: &Machine) -> u64 {
    variables! {
      vars:  0 <= x[machine.joltages.len()] (integer)
    };

    unreachable!()
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| parse_line(&line))
        .collect::<Vec<_>>();

    let part1_answer: u64 = input.iter().map(|machine| light_presses(machine)).sum();
    println!("{part1_answer}");

    let part2_answer: u64 = input.iter().map(|machine| joltage_presses(machine)).sum();
    println!("{part2_answer}");
}
