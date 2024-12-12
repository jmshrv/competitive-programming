use std::{collections::HashSet, io};

type Point = (usize, usize);

fn neighbours(map: &[Vec<char>], point: Point) -> impl Iterator<Item = (Point, char)> + use<'_> {
    [
        (point.0.wrapping_sub(1), point.1),
        (point.0.wrapping_add(1), point.1),
        (point.0, point.1.wrapping_sub(1)),
        (point.0, point.1.wrapping_add(1)),
    ]
    .into_iter()
    .filter_map(|neighbour| match map.get(neighbour.0)?.get(neighbour.1) {
        Some(neighbour_value) => Some((neighbour, *neighbour_value)),
        None => None,
    })
}

fn flood_fill(map: &[Vec<char>], start: Point, results: &mut HashSet<Point>) {
    let goal = map[start.0][start.1];

    results.insert(start);

    let matched_neighbours = neighbours(map, start)
        .filter(|(_, neighbour_value)| goal == *neighbour_value)
        .filter(|(neighbour, _)| results.insert(*neighbour))
        .collect::<Vec<_>>(); // Needed since this chain borrows results

    for (matched_neighbour, _) in matched_neighbours {
        flood_fill(map, matched_neighbour, results);
    }
}

fn perimiter(garden: &HashSet<Point>, bulk_discount: bool) -> usize {
    let mut result = 0;

    for point in garden {
        result += [
            (point.0.wrapping_sub(1), point.1),
            (point.0.wrapping_add(1), point.1),
            (point.0, point.1.wrapping_sub(1)),
            (point.0, point.1.wrapping_add(1)),
        ]
        .iter()
        .filter(|neighbour| !garden.contains(neighbour))
        .count();
    }

    result
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut search_space = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (y, x)))
        .collect::<HashSet<_>>();

    let mut gardens = Vec::new();

    while !search_space.is_empty() {
        let mut garden = HashSet::new();

        flood_fill(&input, *search_space.iter().next().unwrap(), &mut garden);

        for cell in &garden {
            search_space.remove(cell);
        }

        gardens.push(garden);
    }

    let part_one: usize = gardens
        .iter()
        .map(|garden| garden.len() * perimiter(garden, false))
        .sum();

    println!("{part_one}");

    // let part_two: usize = gardens
    //     .iter()
    //     .map(|garden| garden.len() * perimiter(garden, false))
    //     .sum();
}
