use std::{collections::HashMap, io, iter::repeat};

use itertools::Itertools;
use memoize::memoize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn parse_line(line: &str) -> (Vec<Spring>, Vec<usize>) {
    let (condition_records_str, engineer_count_str) =
        line.split_once(' ').expect("Failed to split line!");

    let condition_records = condition_records_str
        .chars()
        .map(|c| match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("Unknown spring {c}!"),
        })
        .collect();

    let engineer_count = engineer_count_str
        .split(',')
        .map(|c| {
            c.parse::<usize>()
                .expect("Failed to parse engineer number!")
        })
        .collect();

    (condition_records, engineer_count)
}

fn valid_arrangements(
    condition_records: &Vec<Spring>,
    engineer_count: &Vec<usize>,
    cache: &mut HashMap<(Vec<Spring>, Vec<usize>), u64>,
) -> u64 {
    if let Some(result) = cache.get(&(condition_records.clone(), engineer_count.clone())) {
        return *result;
    }

    if condition_records.is_empty() {
        if engineer_count.is_empty() {
            return 1;
        }

        return 0;
    }

    if engineer_count.is_empty() {
        if condition_records.contains(&Spring::Damaged) {
            return 0;
        }

        return 1;
    }

    let mut result = 0;

    let first_condition = condition_records
        .first()
        .expect("No first condition somehow?");

    let first_engineer_count = engineer_count
        .first()
        .expect("No first engineer count somehow?");

    if first_condition != &Spring::Damaged {
        result += valid_arrangements(
            &condition_records
                .into_iter()
                .skip(1)
                .map(|spring| *spring)
                .collect::<Vec<_>>(),
            engineer_count,
            cache,
        );
    }

    if first_condition != &Spring::Operational {
        if *first_engineer_count <= condition_records.len()
            && !condition_records
                .iter()
                .take(*first_engineer_count)
                .contains(&Spring::Operational)
            && (*first_engineer_count == condition_records.len()
                || condition_records[*first_engineer_count] != Spring::Damaged)
        {
            let condition_records_split = condition_records
                .iter()
                .skip(*first_engineer_count + 1)
                .map(|spring| *spring)
                .collect::<Vec<_>>();

            let engineer_count_split = engineer_count
                .iter()
                .skip(1)
                .map(|count| *count)
                .collect::<Vec<_>>();

            result += valid_arrangements(&condition_records_split, &engineer_count_split, cache);
        }
    }

    cache.insert((condition_records.clone(), engineer_count.clone()), result);

    result
}

fn unfold_condition_records(condition_records: &Vec<Spring>) -> Vec<Spring> {
    let mut res = vec![];

    for i in 0..5 {
        for condition in condition_records {
            res.push(*condition);
        }

        if i != 4 {
            res.push(Spring::Unknown);
        }
    }

    res
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .map(|line| parse_line(&line))
        .collect::<Vec<_>>();

    let part1_answer: u64 = input
        .par_iter()
        .map(|line| valid_arrangements(&line.0, &line.1, &mut HashMap::new()))
        .sum();

    println!("{part1_answer}");

    let part2_input = input
        .par_iter()
        .map(|line| (unfold_condition_records(&line.0), line.1.repeat(5)))
        .collect::<Vec<_>>();

    let part2_answer: u64 = part2_input
        .par_iter()
        .map(|line| valid_arrangements(&line.0, &line.1, &mut HashMap::new()))
        .sum();

    println!("{part2_answer}");
}
