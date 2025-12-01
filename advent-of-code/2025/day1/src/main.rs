use std::io;

type Position = i32;
type Rotation = (Direction, Position);

enum Direction {
    Left,
    Right,
}

fn parse_rotation(rotation_str: &str) -> Option<(Direction, Position)> {
    let direction = match rotation_str.chars().next()? {
        'L' => Some(Direction::Left),
        'R' => Some(Direction::Right),
        _ => None,
    }?;

    let rotation = rotation_str[1..].parse().ok()?;

    Some((direction, rotation))
}

fn rotate(position: Position, rotation: &Rotation) -> Position {
    match rotation.0 {
        Direction::Left => position - rotation.1,
        Direction::Right => position + rotation.1,
    }
    .rem_euclid(100)
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
            let new_position = rotate(position, rotation);
            let new_ans = if position == 0 { ans + 1 } else { ans };
            (new_ans, new_position)
        })
        .0;

    println!("{part1_answer}");
}
