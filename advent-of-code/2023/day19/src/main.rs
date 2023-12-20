use std::{
    collections::HashMap,
    io,
    ops::{Range, RangeInclusive},
    time::Instant,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till1, take_while1},
    character::complete::char,
    character::complete::u64,
    combinator::{map, map_res, value},
    IResult,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

impl Operation {
    fn evaluate(&self, rating: &Rating) -> bool {
        let left = match self.left_operand {
            Operand::X => rating.x,
            Operand::M => rating.m,
            Operand::A => rating.a,
            Operand::S => rating.s,
        };

        match self.operator {
            Operator::LessThan => left < self.right_operand,
            Operator::GreaterThan => left > self.right_operand,
        }
    }

    fn accepted_range(&self) -> Range<u64> {
        match self.operator {
            Operator::LessThan => 0..self.right_operand,
            Operator::GreaterThan => self.right_operand..4000,
        }
    }
}

#[derive(Debug)]
enum Rule {
    Operation(Operation),
    Destination(Destination),
}

struct Rating {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Rating {
    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

struct ValidRanges {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>,
}

impl ValidRanges {
    fn new() -> Self {
        Self {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }
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

fn parse_rating(rating_str: &str) -> IResult<&str, Rating> {
    let (remaining, _) = char('{')(rating_str)?;

    let (remaining, _) = tag("x=")(remaining)?;

    let (remaining, x) = u64(remaining)?;

    let (remaining, _) = tag(",m=")(remaining)?;

    let (remaining, m) = u64(remaining)?;

    let (remaining, _) = tag(",a=")(remaining)?;

    let (remaining, a) = u64(remaining)?;

    let (remaining, _) = tag(",s=")(remaining)?;

    let (remaining, s) = u64(remaining)?;

    let (remaining, _) = char('}')(remaining)?;

    let rating = Rating { x, m, a, s };

    Ok((remaining, rating))
}

fn eval_rating(
    rating: &Rating,
    workflow: &Vec<Rule>,
    workflows: &HashMap<&str, Vec<Rule>>,
) -> bool {
    for rule in workflow {
        return match rule {
            Rule::Operation(operation) => {
                if operation.evaluate(rating) {
                    match &operation.destination {
                        Destination::Rejected => false,
                        Destination::Accepted => true,
                        Destination::Workflow(workflow_name) => {
                            eval_rating(rating, &workflows[workflow_name.as_str()], workflows)
                        }
                    }
                } else {
                    continue;
                }
            }
            Rule::Destination(destination) => match destination {
                Destination::Rejected => false,
                Destination::Accepted => true,
                Destination::Workflow(workflow_name) => {
                    eval_rating(rating, &workflows[workflow_name.as_str()], workflows)
                }
            },
        };
    }

    unreachable!()
}

fn ranges(mut valid_ranges: ValidRanges, workflow: &Vec<Rule>) -> ValidRanges {
    for rule in workflow {
        match rule {
            Rule::Operation(operation) => {
                let operand_valid = match operation.left_operand {
                    Operand::X => &mut valid_ranges.x,
                    Operand::M => &mut valid_ranges.m,
                    Operand::A => &mut valid_ranges.a,
                    Operand::S => &mut valid_ranges.s,
                };

                let accepted_range = operation.accepted_range();

                match operation.destination {
                    Destination::Rejected => {
                        // let min_rejected = operand_valid.start().max(&accepted_range.start);
                        // let max_rejected = operand_valid.end().min(&accepted_range.end);
                        // *operand_valid = *min_accepted..=*max_accepted;
                    }
                    Destination::Accepted => {
                        let min_accepted = operand_valid.start().max(&accepted_range.start);
                        let max_accepted = operand_valid.end().min(&accepted_range.end);
                        *operand_valid = *min_accepted..=*max_accepted;
                    }
                    Destination::Workflow(workflow_name) => {}
                }
            }
            Rule::Destination(destination) => match destination {
                Destination::Rejected => todo!(),
                Destination::Accepted => todo!(),
                Destination::Workflow(_) => todo!(),
            },
        }
    }

    todo!()
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    let start = Instant::now();

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

    let ratings = ratings_str
        .iter()
        .map(|rating_str| parse_rating(rating_str).expect("Failed to parse rating!").1)
        .collect::<Vec<_>>();

    let parsing_done = Instant::now();

    let part1_answer: u64 = ratings
        .iter()
        .filter(|rating| eval_rating(rating, &workflows["in"], &workflows))
        .map(|accepted_rating| accepted_rating.sum())
        .sum();

    let part1_done = Instant::now();

    println!("{part1_answer}");

    // let part2_ratings = (1..=4000).flat_map(|x| {
    //     (1..=4000)
    //         .map(move |m| (1..=4000).map(move |a| (1..=4000).map(move |s| Rating { x, m, a, s })))
    // });

    println!("{part1_answer}");
    println!("Parsing: {:?}", parsing_done - start);
    println!("Part 1: {:?}", part1_done - parsing_done);
}
