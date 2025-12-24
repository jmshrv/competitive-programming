use std::{
    collections::{HashMap, HashSet},
    io,
};

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Manifold,
    Splitter,
}

fn debug_print(map: &[Vec<Cell>], splits: &HashSet<(usize, usize)>) {
    for (y, line) in map.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if splits.contains(&(y, x)) {
                print!("|");
            } else {
                let char = match cell {
                    Cell::Empty => '.',
                    Cell::Manifold => 'S',
                    Cell::Splitter => '^',
                };

                print!("{char}");
            }
        }

        println!();
    }
}

fn traverse(map: &[Vec<Cell>], mut position: (usize, usize), splits: &mut HashSet<(usize, usize)>) {
    while let Some(cell) = map.get(position.0).and_then(|line| line.get(position.1)) {
        if *cell == Cell::Splitter {
            // No point doing duplicate work - real input takes way too long without this
            if !splits.insert(position) {
                return;
            }

            let left = (position.0, position.1 - 1);
            let right = (position.0, position.1 + 1);

            traverse(map, left, splits);
            traverse(map, right, splits);

            return;
        }

        position.0 += 1;
    }
}

fn universes(map: &[Vec<Cell>], mut position: (usize, usize)) -> u64 {
    let mut splits = HashMap::from([(position.1, 1)]);

    while let Some(row) = map.get(position.0) {
        let split_indices = row
            .iter()
            .enumerate()
            .filter(|(_, cell)| **cell == Cell::Splitter)
            .map(|(x, _)| x)
            .collect::<Vec<_>>();

        for split in &split_indices {
            let Some(&current_split) = splits.get(&split) else {
                continue;
            };

            splits
                .entry(split - 1)
                .and_modify(|count| *count += current_split)
                .or_insert(current_split);

            splits
                .entry(split + 1)
                .and_modify(|count| *count += current_split)
                .or_insert(current_split);
        }

        for split in &split_indices {
            splits.remove(split);
        }

        position.0 += 1;
    }

    splits.values().sum()
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    'S' => Cell::Manifold,
                    '^' => Cell::Splitter,
                    _ => panic!("Invalid character {c}"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start = input
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find(|(_, cell)| **cell == Cell::Manifold)
                .map(|(x, _)| (y, x))
        })
        .unwrap();

    let mut splits = HashSet::new();
    traverse(&input, start, &mut splits);
    println!("{}", splits.len());

    let universe_count = universes(&input, start);
    println!("{universe_count}");
}
