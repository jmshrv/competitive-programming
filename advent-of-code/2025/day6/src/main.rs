use std::io;

fn main() {
    let input = io::stdin().lines().map(Result::unwrap).collect::<Vec<_>>();

    let operands_list = input[..input.len() - 1]
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|operand_str| operand_str.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let operators = input
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<Vec<_>>();

    let mut part1_answer = 0;

    for i in 0..operators.len() {
        let operator = operators[i];

        part1_answer += operands_list
            .iter()
            .map(|row| row[i])
            .reduce(|acc, e| match operator {
                "*" => acc * e,
                "+" => acc + e,
                _ => panic!("Unknown operator {operator}"),
            })
            .unwrap();
    }

    println!("{part1_answer}");
}
