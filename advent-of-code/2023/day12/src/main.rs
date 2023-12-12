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

fn parse_line(line: &str) -> (Vec<Spring>, Vec<u64>) {
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
        .map(|c| c.parse::<u64>().expect("Failed to parse engineer number!"))
        .collect();

    (condition_records, engineer_count)
}

fn valid_arrangements(condition_records: &Vec<Spring>, engineer_count: &Vec<u64>) -> u64 {
    if let Some(unknown_spring_index) = condition_records
        .iter()
        .position(|spring| *spring == Spring::Unknown)
    {
        let mut first_operational = condition_records.clone();
        let mut first_damaged = condition_records.clone();

        first_operational[unknown_spring_index] = Spring::Operational;
        first_damaged[unknown_spring_index] = Spring::Damaged;

        return valid_arrangements(&first_operational, engineer_count)
            + valid_arrangements(&first_damaged, engineer_count);
    }

    // Makes stuff a little bit faster - no point checking if there aren't the right amount of
    // springs
    if condition_records
        .iter()
        .filter(|spring| **spring == Spring::Damaged)
        .count()
        != engineer_count.into_iter().sum::<u64>() as usize
    {
        return 0;
    }

    let mut engineer_index = 0;

    for (key, group) in &condition_records
        .iter()
        .group_by(|element| **element == Spring::Damaged)
    {
        if key {
            if engineer_index >= engineer_count.len() {
                return 0;
            }

            if group.count() != engineer_count[engineer_index] as usize {
                return 0;
            }

            engineer_index += 1;
        }
    }

    if engineer_index != engineer_count.len() {
        return 0;
    }

    1
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
        .iter()
        .map(|line| valid_arrangements(&line.0, &line.1))
        .sum();

    println!("{part1_answer}");

    // let part2_input = input
    //     .par_iter()
    //     .map(|line| (unfold_condition_records(&line.0), line.1.repeat(5)))
    //     .collect::<Vec<_>>();

    // let part2_answer: u64 = part2_input
    //     .par_iter()
    //     .map(|line| valid_arrangements(&line.0, &line.1))
    //     .sum();

    // println!("{part2_answer}");
}
