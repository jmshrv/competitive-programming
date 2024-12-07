use std::io;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn evaluate(self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concatenate => {
                let digits_in_b = ((b as f64).log10() as u32) + 1;

                a * 10_u64.pow(digits_in_b) + b
            }
        }
    }
}

fn is_equation_valid(expected_answer: u64, equations: &[u64], operators: &[Operator]) -> bool {
    // If we've gone past the expected answer, no point recursing further
    if equations[0] > expected_answer {
        return false;
    }

    if equations.len() == 1 {
        return expected_answer == equations[0];
    }

    operators
        .iter()
        .map(|operator| operator.evaluate(equations[0], equations[1]))
        .any(|res| {
            is_equation_valid(
                expected_answer,
                &[&[res], &equations[2..]].concat(),
                operators,
            )
        })
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(answer_str, operators_str)| {
            (
                answer_str.parse::<u64>().unwrap(),
                operators_str
                    .split(' ')
                    .map(|operator| operator.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let part_one: u64 = input
        .iter()
        .filter(|(answer, equations)| {
            is_equation_valid(*answer, &equations, &[Operator::Add, Operator::Multiply])
        })
        .map(|(answer, _)| answer)
        .sum();

    println!("{part_one}");

    let part_two: u64 = input
        .iter()
        .filter(|(answer, equations)| {
            is_equation_valid(
                *answer,
                &equations,
                &[Operator::Add, Operator::Multiply, Operator::Concatenate],
            )
        })
        .map(|(answer, _)| answer)
        .sum();

    println!("{part_two}");
}
