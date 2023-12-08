use std::{collections::HashMap, io};

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

enum Direction {
    Left,
    Right,
}

fn parse_directions(line: &str) -> Result<Vec<Direction>, char> {
    line.chars()
        .map(|c| match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(c),
        })
        .collect()
}

fn parse_destination(line: &str) -> (&str, (&str, &str)) {
    let destination = &line[..3];

    let left = &line[7..10];
    let right = &line[12..15];

    (destination, (left, right))
}

fn traverse<'a>(
    destinations: &'a HashMap<&'a str, (&'a str, &'a str)>,
    current_position: &'a str,
    directions: &'a Vec<Direction>,
    steps: usize,
) -> &'a str {
    let destination = destinations
        .get(current_position)
        .expect(&format!("Failed to find destination {current_position}"));

    let direction = &directions[steps % directions.len()];

    match direction {
        Direction::Left => destination.0,
        Direction::Right => destination.1,
    }
}

fn traverse_to<F>(
    start: &str,
    destinations: &HashMap<&str, (&str, &str)>,
    directions: &Vec<Direction>,
    destination: F,
) -> usize
where
    F: Fn(&str) -> bool,
{
    let mut current_position = start;
    let mut steps = 0;

    while !destination(current_position) {
        current_position = traverse(destinations, current_position, directions, steps);
        steps += 1;
    }

    steps
}

fn traverse_part2_naive(
    destinations: &HashMap<&str, (&str, &str)>,
    directions: &Vec<Direction>,
) -> usize {
    let mut positions = destinations
        .keys()
        .filter(|destination| destination.ends_with('A'))
        .map(|destination| *destination)
        .collect::<Vec<_>>();

    let mut steps = 0;

    while !positions.iter().all(|position| position.ends_with('Z')) {
        // for position in &mut positions {
        //     *position = traverse(destinations, *position, directions, steps);
        // }

        positions
            .par_iter_mut()
            .for_each(|position| *position = traverse(destinations, *position, directions, steps));

        steps += 1;
    }

    steps
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn traverse_part2(
    destinations: &HashMap<&str, (&str, &str)>,
    directions: &Vec<Direction>,
) -> usize {
    let starts = destinations
        .keys()
        .filter(|destination| destination.ends_with('A'))
        .map(|destination| *destination)
        .collect::<Vec<_>>();

    let steps = starts
        .iter()
        .map(|start| traverse_to(start, destinations, directions, |pos| pos.ends_with('Z')))
        .reduce(lcm)
        .expect("No starts?");

    steps
}

fn main() {
    let lines = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    let directions = parse_directions(&lines.first().expect("No first line?"))
        .expect("Failed to parse directions");

    let destinations = lines
        .iter()
        .skip(2)
        .map(|line| parse_destination(line))
        .collect::<HashMap<_, _>>();

    let part1_answer = traverse_to("AAA", &destinations, &directions, |pos| pos == "ZZZ");

    println!("{part1_answer}");

    let part2_answer = traverse_part2(&destinations, &directions);

    println!("{part2_answer}");
}
