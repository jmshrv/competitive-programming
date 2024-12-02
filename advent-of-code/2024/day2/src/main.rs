use std::{cmp::Ordering, io};

use itertools::Itertools;

fn is_reactor_valid(reactor: &[i32]) -> bool {
    let is_ordering_valid = reactor
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| a.cmp(b))
        .filter(|ord| *ord != Ordering::Equal)
        .all_equal();

    let is_difference_valid = reactor
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| (a - b).abs())
        .all(|difference| difference >= 1 && difference <= 3);

    is_ordering_valid && is_difference_valid
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .iter()
        .map(|line| {
            line.split(' ')
                .map(|entry| entry.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part_one = input
        .iter()
        .filter(|reactor| is_reactor_valid(reactor))
        .count();

    println!("{part_one}");

    let part_two = input
        .iter()
        .filter(|reactor| {
            if is_reactor_valid(reactor) {
                return true;
            }

            for i in 0..reactor.len() {
                let reactor_without_level = [&reactor[..i], &reactor[(i + 1)..]].concat();
                if is_reactor_valid(&reactor_without_level) {
                    return true;
                }
            }

            return false;
        })
        .count();

    println!("{part_two}");
}
