use std::{collections::HashMap, io};

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till1, take_while1},
    character::complete::char,
    character::complete::u64,
    combinator::{map, map_res, value},
    IResult,
};

#[derive(Clone, Copy, Debug)]
enum Operand {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Debug)]
enum Destination {
    Rejected,
    Accepted,
    Workflow(String),
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
struct Operation {
    left_operand: Operand,
    operator: Operator,
    right_operand: u64,
    destination: Destination,
}

#[derive(Debug)]
enum Rule {
    Operation(Operation),
    Destination(Destination),
}

fn parse_destination(destination_str: &str) -> IResult<&str, Destination> {
    map(
        take_while1(|c: char| c.is_alphabetic()),
        |s: &str| match s {
            "A" => Destination::Accepted,
            "R" => Destination::Rejected,
            _ => Destination::Workflow(s.to_string()),
        },
    )(destination_str)
}

fn parse_operation(operation_str: &str) -> IResult<&str, Operation> {
    let (remaining, left_operand) = alt((
        value(Operand::X, char('x')),
        value(Operand::M, char('m')),
        value(Operand::A, char('a')),
        value(Operand::S, char('s')),
    ))(operation_str)?;

    let (remaining, operator) = alt((
        value(Operator::LessThan, char('<')),
        value(Operator::GreaterThan, char('>')),
    ))(remaining)?;

    let (remaining, right_operand) = u64(remaining)?;

    let (remaining, _) = char(':')(remaining)?;

    // let (remaining, destination) = alt((
    //     value(Destination::Accepted, char('A')),
    //     value(Destination::Rejected, char('R')),
    //     value(
    //         Destination::Workflow(remaining.to_string()),
    //         take_while1(|_| true),
    //     ),
    // ))(remaining)?;

    let (remaining, destination) = parse_destination(remaining)?;

    let operation = Operation {
        left_operand,
        operator,
        right_operand,
        destination,
    };

    Ok((remaining, operation))
}

fn parse_rule(condition_str: &str) -> IResult<&str, Rule> {
    alt((
        map(parse_operation, |res| Rule::Operation(res)),
        map(parse_destination, |res| Rule::Destination(res)),
    ))(condition_str)
}

fn parse_workflow(workflow_str: &str) -> IResult<&str, (&str, Vec<Rule>)> {
    let (remaining, name) = take_till1(|c| c == '{')(workflow_str)?;

    let (remaining, _) = char('{')(remaining)?;

    let (remaining, conditions_str) = take_till1(|c| c == '}')(remaining)?;

    let rules = conditions_str
        .split(',')
        .map(|condition_str| parse_rule(condition_str).expect("Failed to parse rule!").1)
        .collect::<Vec<_>>();

    Ok((remaining, (name, rules)))
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    let (workflows_str, ratings_str_with_newline) = input.split_at(
        input
            .iter()
            .position(|line| line.is_empty())
            .expect("Failed to find empty line!"),
    );
    let ratings_str = &ratings_str_with_newline[1..];

    let workflows = workflows_str
        .iter()
        .map(|workflow_str| {
            parse_workflow(workflow_str)
                .expect("Failed to parse workflow!")
                .1
        })
        .collect::<HashMap<_, _>>();

    for workflow in workflows {
        println!("{:?}", workflow);
    }
}
