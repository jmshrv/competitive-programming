use std::io;

fn parse_line(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .map(|value| value.parse::<i64>().expect("Failed to parse value"))
        .collect()
}

fn diffs(result: &Vec<i64>) -> Vec<i64> {
    result
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

fn next(result: &Vec<i64>) -> i64 {
    let diffs = diffs(result);

    let result_last = *result.last().expect("Result is empty?");

    if diffs.iter().all(|diff| *diff == 0) {
        result_last
    } else {
        result_last + next(&diffs)
    }
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    let results = input
        .iter()
        .map(|line| parse_line(&line))
        .collect::<Vec<_>>();

    let part1_answer: i64 = results.iter().map(|result| next(&result)).sum();

    println!("{part1_answer}");

    let part2_answer: i64 = results
        .into_iter()
        .map(|result| next(&result.into_iter().rev().collect()))
        .sum();

    println!("{part2_answer}");
}
