use std::{collections::HashSet, io};

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Guard,
    Empty,
    Obstacle,
}

fn traverse(
    map: &Vec<Vec<Cell>>,
    mut guard_pos: (isize, isize),
    guard_vector: &mut (isize, isize),
) -> Option<(isize, isize)> {
    let start_pos = guard_pos;

    guard_pos.0 += guard_vector.0;
    guard_pos.1 += guard_vector.1;

    if let Some(row) = map.get(guard_pos.0 as usize) {
        if let Some(cell) = row.get(guard_pos.1 as usize) {
            if *cell == Cell::Obstacle {
                guard_pos = start_pos;

                *guard_vector = match guard_vector {
                    (1, 0) => (0, -1),
                    (0, 1) => (1, 0),
                    (-1, 0) => (0, 1),
                    (0, -1) => (-1, 0),
                    _ => panic!("Invalid vector {guard_vector:?}"),
                }
            }

            return Some(guard_pos);
        }
    }

    None
}

fn positions_visited(map: &Vec<Vec<Cell>>) -> HashSet<(isize, isize)> {
    let mut guard_vector = (-1, 0);

    let mut guard_pos = map
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find(|(_, cell)| **cell == Cell::Guard)
                .map(|(x, _)| (y as isize, x as isize))
        })
        .next()
        .unwrap();

    let mut output = HashSet::from([guard_pos]);

    while let Some(new_pos) = traverse(map, guard_pos, &mut guard_vector) {
        guard_pos = new_pos;
        output.insert(new_pos);
    }

    output
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Cell::Empty,
                    '#' => Cell::Obstacle,
                    '^' => Cell::Guard,
                    _ => panic!("Invalid cell {char}!"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part_one = positions_visited(&input).len();

    println!("{part_one}");
}
