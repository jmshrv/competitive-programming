use std::io;

use lazy_regex::regex;
use nalgebra::{Matrix2, Vector2};

#[derive(Debug)]
struct Statement {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

fn parse_statement(input: &str) -> Option<Statement> {
    let regex = regex!(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)"
    );

    let captures = regex.captures(input)?;

    if captures.len() != 7 {
        return None;
    }

    let ax = captures.get(1)?.as_str().parse::<u64>().ok()?;
    let ay = captures.get(2)?.as_str().parse::<u64>().ok()?;

    let bx = captures.get(3)?.as_str().parse::<u64>().ok()?;
    let by = captures.get(4)?.as_str().parse::<u64>().ok()?;

    let prizex = captures.get(5)?.as_str().parse::<u64>().ok()?;
    let prizey = captures.get(6)?.as_str().parse::<u64>().ok()?;

    Some(Statement {
        button_a: (ax, ay),
        button_b: (bx, by),
        prize: (prizex, prizey),
    })
}

fn button_presses(statement: &Statement) -> Option<u64> {
    let problems = Matrix2::new(
        statement.button_a.0 as f64,
        statement.button_b.0 as f64,
        statement.button_a.1 as f64,
        statement.button_b.1 as f64,
    );

    let goal = Vector2::new(statement.prize.0 as f64, statement.prize.1 as f64);
    let decomp = problems.lu();

    let result = decomp.solve(&goal)?;

    let x_remainder = result.x.fract();
    let y_remainder = result.y.fract();

    if (x_remainder > 0.001 && x_remainder < 0.999) || (y_remainder > 0.001 && y_remainder < 0.999)
    {
        return None;
    }

    Some(result.x.round() as u64 * 3 + result.y.round() as u64)
}

fn main() {
    let input = io::read_to_string(io::stdin())
        .unwrap()
        .split("\n\n")
        .map(|statement| parse_statement(statement).unwrap())
        .collect::<Vec<_>>();

    let part_one: u64 = input.iter().filter_map(button_presses).sum();

    println!("{part_one}");

    let part_two: u64 = input
        .iter()
        .map(|statement| Statement {
            button_a: statement.button_a,
            button_b: statement.button_b,
            prize: (
                statement.prize.0 + 10000000000000,
                statement.prize.1 + 10000000000000,
            ),
        })
        .filter_map(|statement| button_presses(&statement))
        .sum();

    println!("{part_two}");
}
