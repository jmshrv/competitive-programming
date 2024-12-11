use std::{collections::HashSet, io};

type Position = (usize, usize);

fn positions_at_value(map: &Vec<Vec<u32>>, value: u32) -> impl Iterator<Item = Position> + use<'_> {
    map.iter().enumerate().flat_map(move |(y, row)| {
        row.iter()
            .enumerate()
            .filter_map(move |(x, pos)| if *pos == value { Some((y, x)) } else { None })
    })
}

fn dfs(
    map: &Vec<Vec<u32>>,
    (position_y, position_x): Position,
    score: usize,
    visited: &mut Option<HashSet<Position>>,
) -> usize {
    if position_y > map.len() - 1 {
        return 0;
    }

    if position_x > map[position_y].len() - 1 {
        return 0;
    }

    let height = map[position_y][position_x];

    if height == 9 {
        if let Some(visited) = visited {
            return visited.insert((position_y, position_x)) as usize;
        }

        return 1;
    }

    let neighbours = [
        (position_y - 1, position_x),
        (position_y + 1, position_x),
        (position_y, position_x - 1),
        (position_y, position_x + 1),
    ]
    .into_iter()
    .filter(|(neighbour_y, neighbour_x)| {
        if *neighbour_y > map.len() - 1 {
            return false;
        }

        if *neighbour_x > map[position_y].len() - 1 {
            return false;
        }

        map[*neighbour_y][*neighbour_x] - height == 1
    })
    .collect::<Vec<_>>();

    neighbours
        .iter()
        .map(|neighbour| dfs(map, *neighbour, score, visited))
        .sum()
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|pos| pos.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let trailheads = positions_at_value(&input, 0).collect::<Vec<_>>();

    let part_one: usize = trailheads
        .iter()
        .map(|trailhead| dfs(&input, *trailhead, 0, &mut Some(HashSet::new())))
        .sum();

    println!("{part_one}");

    let part_two: usize = trailheads
        .iter()
        .map(|trailhead| dfs(&input, *trailhead, 0, &mut None))
        .sum();

    println!("{part_two}");
}
