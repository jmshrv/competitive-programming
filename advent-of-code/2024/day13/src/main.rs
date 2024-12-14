use std::{cmp::Ordering, collections::BinaryHeap, io};

use lazy_regex::regex;

#[derive(Debug)]
struct Statement {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    tokens: u64,
    x: u64,
    y: u64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.tokens.cmp(&self.tokens)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
    let mut queue = BinaryHeap::from([State {
        tokens: 0,
        x: 0,
        y: 0,
    }]);

    while let Some(node) = queue.pop() {
        if node.x == statement.prize.0 && node.y == statement.prize.1 {
            return Some(node.tokens);
        }

        let a_next = (node.x + statement.button_a.0, node.y + statement.button_a.1);
        let b_next = (node.x + statement.button_b.0, node.y + statement.button_b.1);

        // println!("{a_next:?}");
        // println!("{b_next:?}");

        if a_next.0 <= statement.prize.0 && a_next.1 <= statement.prize.1 {
            queue.push(State {
                tokens: node.tokens + 3,
                x: a_next.0,
                y: a_next.1,
            });
        }

        if b_next.0 <= statement.prize.0 && b_next.1 <= statement.prize.1 {
            queue.push(State {
                tokens: node.tokens + 1,
                x: b_next.0,
                y: b_next.0,
            });
        }
    }

    None
}

fn main() {
    let input = io::read_to_string(io::stdin())
        .unwrap()
        .split("\n\n")
        .map(|statement| parse_statement(statement).unwrap())
        .collect::<Vec<_>>();

    for statement in input {
        println!("{:?}", button_presses(&statement));
    }
}
