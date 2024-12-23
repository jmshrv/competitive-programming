use itertools::Itertools;

use std::{
    collections::{HashMap, HashSet},
    io,
};

fn mix(secret_number: u64, value: u64) -> u64 {
    value ^ secret_number
}

fn prune(secret_number: u64) -> u64 {
    secret_number % 16777216
}

fn evolve(mut secret_number: u64) -> u64 {
    secret_number = mix(secret_number, secret_number * 64);
    secret_number = prune(secret_number);

    secret_number = mix(secret_number, secret_number / 32);
    secret_number = prune(secret_number);

    secret_number = mix(secret_number, secret_number * 2048);

    prune(secret_number)
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let part_one: u64 = input
        .iter()
        .map(|initial_secret| {
            let mut answer = *initial_secret;

            for _ in 0..2000 {
                answer = evolve(answer);
            }

            answer
        })
        .sum();

    println!("{part_one}");

    let mut profits = HashMap::new();

    let sellers = input
        .iter()
        .map(|initial_secret| {
            let mut secret = *initial_secret;

            let mut answer = vec![secret % 10];

            for _ in 0..2000 {
                secret = evolve(secret);
                answer.push(secret % 10);
            }

            answer
        })
        .collect::<Vec<_>>();

    for prices in &sellers {
        let price_diffs = prices
            .iter()
            .tuple_windows()
            .map(|(a, b)| (*b, *a as i64 - *b as i64));

        let mut seen = HashSet::new();

        for window in price_diffs.tuple_windows::<(_, _, _, _)>() {
            let differences = (window.0 .1, window.1 .1, window.2 .1, window.3 .1);

            if !seen.insert(differences) {
                continue;
            }

            let price = window.3 .0;

            profits
                .entry(differences)
                .and_modify(|diff_price| *diff_price += price)
                .or_insert(price);
        }
    }

    let part_two = profits.values().max().unwrap();

    println!("{part_two}");
}
