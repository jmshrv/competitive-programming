use std::{collections::BTreeSet, io};

use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Guard,
    Empty,
    Obstacle,
}

fn obstacle_rotate(
    guard_pos: &mut (isize, isize),
    start_pos: (isize, isize),
    guard_vector: &mut (isize, isize),
) {
    *guard_pos = start_pos;

    *guard_vector = match guard_vector {
        (1, 0) => (0, -1),
        (0, 1) => (1, 0),
        (-1, 0) => (0, 1),
        (0, -1) => (-1, 0),
        _ => panic!("Invalid vector {guard_vector:?}"),
    }
}

fn traverse(
    map: &Vec<Vec<Cell>>,
    mut guard_pos: (isize, isize),
    guard_vector: &mut (isize, isize),
    extra_obstacle: Option<(isize, isize)>,
) -> Option<(isize, isize)> {
    let start_pos = guard_pos;

    guard_pos.0 += guard_vector.0;
    guard_pos.1 += guard_vector.1;

    if let Some(extra_obstacle) = extra_obstacle {
        if guard_pos.0 == extra_obstacle.0 && guard_pos.1 == extra_obstacle.1 {
            obstacle_rotate(&mut guard_pos, start_pos, guard_vector);

            return Some(guard_pos);
        }
    }

    if let Some(row) = map.get(guard_pos.0 as usize) {
        if let Some(cell) = row.get(guard_pos.1 as usize) {
            if *cell == Cell::Obstacle {
                obstacle_rotate(&mut guard_pos, start_pos, guard_vector);
            }

            return Some(guard_pos);
        }
    }

    None
}

fn positions_visited(map: &Vec<Vec<Cell>>) -> BTreeSet<(isize, isize)> {
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

    let mut output = BTreeSet::from([guard_pos]);

    while let Some(new_pos) = traverse(map, guard_pos, &mut guard_vector, None) {
        guard_pos = new_pos;
        output.insert(new_pos);
    }

    output
}

fn loop_count(map: &Vec<Vec<Cell>>, guard_visited: &BTreeSet<(isize, isize)>) -> usize {
    let start_pos = map
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

    guard_visited
        .par_iter()
        .filter(|(y, x)| {
            let mut guard_pos = start_pos;
            let mut guard_vector = (-1, 0);

            let mut traversed = BTreeSet::from([(guard_pos, guard_vector)]);

            while let Some(new_pos) = traverse(&map, guard_pos, &mut guard_vector, Some((*y, *x))) {
                guard_pos = new_pos;

                if !traversed.insert((new_pos, guard_vector)) {
                    return true;
                }
            }

            return false;
        })
        .count()
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

    let guard_visited = positions_visited(&input);
    let part_one = guard_visited.len();

    println!("{part_one}");

    let part_two = loop_count(&input, &guard_visited);

    println!("{part_two}");
}
