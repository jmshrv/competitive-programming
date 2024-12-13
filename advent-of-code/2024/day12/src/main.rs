use std::{collections::BTreeSet, io};

type Point = (isize, isize);

fn neighbours(map: &[Vec<char>], point: Point) -> impl Iterator<Item = (Point, char)> + use<'_> {
    [
        (point.0.wrapping_sub(1), point.1),
        (point.0.wrapping_add(1), point.1),
        (point.0, point.1.wrapping_sub(1)),
        (point.0, point.1.wrapping_add(1)),
    ]
    .into_iter()
    .filter_map(
        |neighbour| match map.get(neighbour.0 as usize)?.get(neighbour.1 as usize) {
            Some(neighbour_value) => Some((neighbour, *neighbour_value)),
            None => None,
        },
    )
}

fn flood_fill(map: &[Vec<char>], start: Point, results: &mut BTreeSet<Point>) {
    let goal = map[start.0 as usize][start.1 as usize];

    results.insert(start);

    let matched_neighbours = neighbours(map, start)
        .filter(|(_, neighbour_value)| goal == *neighbour_value)
        .filter(|(neighbour, _)| results.insert(*neighbour))
        .collect::<Vec<_>>(); // Needed since this chain borrows results

    for (matched_neighbour, _) in matched_neighbours {
        flood_fill(map, matched_neighbour, results);
    }
}

fn perimiter(garden: &BTreeSet<Point>, bulk_discount: bool) -> (usize, usize) {
    let mut fences: Vec<Point> = Vec::new();

    for point in garden {
        fences.extend(
            [
                (point.0.wrapping_sub(1), point.1),
                (point.0, point.1.wrapping_sub(1)),
                (point.0.wrapping_add(1), point.1),
                (point.0, point.1.wrapping_add(1)),
            ]
            .iter()
            .filter(|neighbour| !garden.contains(neighbour)),
        );
    }

    if bulk_discount {
        // Fill in corners, since the fence detection doesn't store outer corners
        for point in garden {
            fences.extend(
                [
                    (point.0 - 1, point.1 - 1),
                    (point.0 + 1, point.1 + 1),
                    (point.0 + 1, point.1 - 1),
                    (point.0 - 1, point.1 + 1),
                ]
                .iter()
                .filter(|neighbour| !garden.contains(neighbour)),
            );
        }

        let corners = fences
            .iter()
            .filter(|point| {
                [
                    // fences.contains(&(point.0 - 1, point.1 - 1)),
                    // fences.contains(&(point.0 + 1, point.1 + 1)),
                    // fences.contains(&(point.0 + 1, point.1 - 1)),
                    // fences.contains(&(point.0 - 1, point.1 + 1)),
                    fences.contains(&(point.0 + 1, point.1))
                        && fences.contains(&(point.0, point.1 + 1)),
                    fences.contains(&(point.0 + 1, point.1))
                        && fences.contains(&(point.0, point.1 - 1)),
                    fences.contains(&(point.0 - 1, point.1))
                        && fences.contains(&(point.0, point.1 - 1)),
                    fences.contains(&(point.0 - 1, point.1))
                        && fences.contains(&(point.0, point.1 + 1)),
                ]
                .iter()
                .filter(|x| **x)
                .count()
                    == 1
            })
            .collect::<BTreeSet<_>>();

        let area = garden
            .iter()
            .filter(|(point_y, point_x)| point_y % 3 == 0 && point_x % 3 == 0)
            .count();

        return (area, corners.len());
    }

    (garden.len(), fences.len())
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
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, _)| (y as isize, x as isize))
        })
        .collect::<BTreeSet<_>>();

    let mut gardens = Vec::new();

    while !search_space.is_empty() {
        let mut garden = BTreeSet::new();

        flood_fill(&input, *search_space.first().unwrap(), &mut garden);

        for cell in &garden {
            search_space.remove(cell);
        }

        gardens.push(garden);
    }

    let part_one: usize = gardens
        .iter()
        .map(|garden| perimiter(garden, false))
        .map(|(area, perimiter)| area * perimiter)
        .sum();

    println!("{part_one}");

    let scaled_input = input
        .iter()
        .flat_map(|row| [row, row, row])
        .map(|row| {
            row.iter()
                .flat_map(|cell| [*cell, *cell, *cell])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut search_space = scaled_input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, _)| (y as isize, x as isize))
        })
        .collect::<BTreeSet<_>>();

    let mut gardens = Vec::new();

    while !search_space.is_empty() {
        let mut garden = BTreeSet::new();

        flood_fill(&scaled_input, *search_space.first().unwrap(), &mut garden);

        for cell in &garden {
            search_space.remove(cell);
        }

        gardens.push(garden);
    }

    let part_two: usize = gardens
        .iter()
        .map(|garden| perimiter(garden, true))
        .map(|(area, perimiter)| area * perimiter)
        .sum();

    println!("{part_two}");
}
