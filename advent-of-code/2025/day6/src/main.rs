use std::io;

use itertools::Itertools;

fn main() {
    let input = io::stdin().lines().map(Result::unwrap).collect::<Vec<_>>();

    let problem_indices = input
        .last()
        .unwrap()
        .char_indices()
        .filter(|(_, c)| *c == '*' || *c == '+')
        .collect::<Vec<_>>();

    let operands_list = input[..input.len() - 1]
        .iter()
        .map(|line| {
            problem_indices
                .iter()
                .map(|(i, _)| *i)
                .chain([line.len()]) // we need to collect the last row too
                .tuple_windows()
                .map(|(from, to)| &line[from..to])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let operators = input
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<Vec<_>>();

    let mut part1_answer = 0;
    let mut part2_answer = 0;

    for i in 0..operators.len() {
        let operator = operators[i];

        part1_answer += operands_list
            .iter()
            .map(|row| row[i].trim_ascii().parse::<u64>().unwrap())
            .reduce(|acc, e| match operator {
                "*" => acc * e,
                "+" => acc + e,
                _ => panic!("Unknown operator {operator}"),
            })
            .unwrap();

        let digit_count = operands_list
            .iter()
            .map(|row| row[i])
            .map(|e| e.chars().count())
            .max()
            .unwrap();

        part2_answer += (0..digit_count)
            .rev()
            .filter_map(|digit_index| {
                operands_list
                    .iter()
                    .map(|row| row[i])
                    .filter_map(|e| {
                        let char = e.chars().nth(digit_index).unwrap();

                        // Whitespace chars will return None here
                        char.to_digit(10)
                    })
                    .map(|digit| digit as u64)
                    .reduce(|acc, e| acc * 10 + e)
            })
            .reduce(|acc, e| match operator {
                "*" => acc * e,
                "+" => acc + e,
                _ => panic!("Unknown operator {operator}"),
            })
            .unwrap();
    }

    println!("{part1_answer}");
    println!("{part2_answer}");
}
