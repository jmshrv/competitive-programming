use std::io;

use lazy_regex::regex;

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

fn button_presses(statement: &Statement) -> Option<(u64, u64)> {
    let mut tokens = 0;

    // Dijkstra :D

    todo!()
}

fn main() {
    let input = io::read_to_string(io::stdin())
        .unwrap()
        .split("\n\n")
        .map(|statement| parse_statement(statement).unwrap())
        .collect::<Vec<_>>();
}
