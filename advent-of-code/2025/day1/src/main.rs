use std::{
    cmp::{max, min},
    io,
};

use itertools::Either;

type Rotation = (Direction, i32);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn parse_rotation(rotation_str: &str) -> Option<(Direction, i32)> {
    let direction = match rotation_str.chars().next()? {
        'L' => Some(Direction::Left),
        'R' => Some(Direction::Right),
        _ => None,
    }?;

    let rotation = rotation_str[1..].parse().ok()?;

    Some((direction, rotation))
}

fn next_step(position: i32, rotation: &Rotation) -> i32 {
    match rotation.0 {
        Direction::Left => position - rotation.1,
        Direction::Right => position + rotation.1,
    }
}

fn full_rotations(position: i32, rotation: &Rotation) -> usize {
    let dest = next_step(position, rotation);

    let range = min(dest, position)..=max(dest, position);

    let iter = match rotation.0 {
        Direction::Left => Either::Left(range.rev()),
        Direction::Right => Either::Right(range),
    };

    iter
        .skip(1) // We don't want to count the first step, only the clicks!
        .map(|pos| pos.rem_euclid(100))
        .filter(|pos| *pos == 0)
        .count()
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| parse_rotation(&line))
        .map(Option::unwrap)
        .collect::<Vec<_>>();

    let part1_answer = input
        .iter()
        .fold((0, 50), |(ans, position), rotation| {
            let new_position = next_step(position, rotation).rem_euclid(100);
            let new_ans = if position == 0 { ans + 1 } else { ans };
            (new_ans, new_position)
        })
        .0;

    println!("{part1_answer}");

    let part2_answer = input
        .iter()
        .fold((0, 50), |(ans, position), rotation| {
            let new_position = next_step(position, rotation).rem_euclid(100);
            (ans + full_rotations(position, rotation), new_position)
        })
        .0;

    println!("{part2_answer}");
}
