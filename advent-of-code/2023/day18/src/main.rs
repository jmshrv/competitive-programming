use std::{io, time::Instant};

use nom::{
    branch::alt, bytes::complete::take, character::complete::char, character::complete::i64,
    combinator::value, IResult,
};
use num::Integer;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct DigInstruction {
    direction: Direction,
    distance: isize,
    part2_distance: isize,
    part2_direction: Direction,
}

impl DigInstruction {
    fn vector(&self, part2: bool) -> (isize, isize) {
        if part2 {
            match self.part2_direction {
                Direction::Left => (-self.part2_distance, 0),
                Direction::Right => (self.part2_distance, 0),
                Direction::Up => (0, -self.part2_distance),
                Direction::Down => (0, self.part2_distance),
            }
        } else {
            match self.direction {
                Direction::Left => (-self.distance, 0),
                Direction::Right => (self.distance, 0),
                Direction::Up => (0, -self.distance),
                Direction::Down => (0, self.distance),
            }
        }
    }
}

fn parse_line(line: &str) -> IResult<&str, DigInstruction> {
    // let (remaining, direction_char) = take(1_usize)(line)?

    let (remaining, direction) = alt((
        value(Direction::Up, char('U')),
        value(Direction::Down, char('D')),
        value(Direction::Left, char('L')),
        value(Direction::Right, char('R')),
    ))(line)?;

    let (remaining, _) = take(1_usize)(remaining)?;

    // let (remaining, distance) = take_while1(|c: char| c.is_ascii_digit())(remaining)?;
    let (remaining, distance) = i64(remaining)?;

    let (remaining, _) = take(3_usize)(remaining)?;

    let (remaining, part2_distance_hex) = take(5_usize)(remaining)?;

    let part2_distance =
        isize::from_str_radix(part2_distance_hex, 16).expect("Failed hex to dec conversion!");

    let (remaining, part2_direction) = alt((
        value(Direction::Right, char('0')),
        value(Direction::Down, char('1')),
        value(Direction::Left, char('2')),
        value(Direction::Up, char('3')),
    ))(remaining)?;

    let dig_instruction = DigInstruction {
        direction,
        distance: (distance as isize),
        part2_distance,
        part2_direction,
    };

    Ok((remaining, dig_instruction))
}

fn verticies(input: &Vec<DigInstruction>, part2: bool) -> Vec<(isize, isize)> {
    let mut position = (0, 0);
    let mut res = vec![position];

    for instruction in input {
        let vector = instruction.vector(part2);
        position.0 += vector.0;
        position.1 += vector.1;
        res.push(position);
    }

    res
}

fn shoelace(verticies: &Vec<(isize, isize)>) -> isize {
    let mut res = 0;

    for i in 0..verticies.len() - 1 {
        let vert_i = verticies[i];
        let vert_i_next = verticies[i + 1];

        // res += vert_i.0 * vert_i_next.1 - vert_i_next.0 * vert_i.1;
        res += (vert_i.1 + vert_i_next.1) * (vert_i.0 - vert_i_next.0);
    }

    res
}

fn line_points((x0, y0): (isize, isize), (x1, y1): (isize, isize)) -> isize {
    (x1 - x0).abs().gcd(&(y1 - y0).abs())
}

fn boundary_points(verticies: &Vec<(isize, isize)>) -> isize {
    verticies
        .windows(2)
        .map(|window| line_points(window[0], window[1]))
        .sum()
}

fn area(verticies: &Vec<(isize, isize)>) -> isize {
    (shoelace(verticies) + boundary_points(verticies)) / 2 + 1
}

fn main() {
    let start = Instant::now();

    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .map(|line| parse_line(&line).expect("Failed to parse line!").1)
        .collect::<Vec<_>>();

    let input_done = Instant::now();

    let part1_answer = area(&verticies(&input, false));
    let part1_done = Instant::now();

    let part2_answer = area(&verticies(&input, true));
    let part2_done = Instant::now();

    println!("{part1_answer}");
    println!("{part2_answer}");

    println!("Parsing: {:?}", input_done - start);
    println!("Part One: {:?}", part1_done - input_done);
    println!("Part Two: {:?}", part2_done - part1_done);
}
