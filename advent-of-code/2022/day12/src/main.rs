use std::{
    io,
    sync::{atomic::AtomicUsize, Mutex},
};

use pathfinding::prelude::bfs;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

fn find_char(map: &Vec<String>, target: char) -> Vec<(usize, usize)> {
    let mut cells = Vec::new();

    for i in 0..map.len() {
        let chars: Vec<_> = map[i].chars().collect();
        for j in 0..chars.len() {
            if chars[j] == target {
                cells.push((j, i));
            }
        }
    }

    cells
}

fn get_char_index(map: &Vec<String>, pos: (usize, usize)) -> Option<((usize, usize), char)> {
    if let Some(char) = map.get(pos.1)?.chars().nth(pos.0) {
        return Some(((pos.0, pos.1), char));
    }

    None
}

fn get_neighbours(map: &Vec<String>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    let height = map[pos.1].chars().nth(pos.0).unwrap();
    let height_start_end = match height {
        'S' => 'a',
        'E' => 'z',
        _ => height,
    };

    let adjacent = [
        // Up
        if pos.1 == 0 {
            None
        } else {
            get_char_index(map, (pos.0, pos.1 - 1))
        },
        // Down
        get_char_index(map, (pos.0, pos.1 + 1)),
        // Left
        if pos.0 == 0 {
            None
        } else {
            get_char_index(map, (pos.0 - 1, pos.1))
        },
        // Right
        get_char_index(map, (pos.0 + 1, pos.1)),
    ];

    for cell in adjacent
        .iter()
        .filter(|cell_res| cell_res.is_some())
        .map(|cell_res| cell_res.unwrap())
    {
        // if cell.1 == 'E' || (height == 'S' && cell.1 == 'a') {
        //     neighbours.push(cell.0);
        //     continue;
        // }

        let cell_start_end = match cell.1 {
            'S' => 'a',
            'E' => 'z',
            _ => cell.1,
        };

        let cell_diff = cell_start_end as i8 - height_start_end as i8;

        if cell_diff <= 1 {
            neighbours.push(cell.0);
        }
    }

    neighbours
}

fn main() {
    let input: Vec<String> = io::stdin()
        .lines()
        .map(|line_res| line_res.unwrap())
        .collect();

    let start_binding = find_char(&input, 'S');
    let start = start_binding.first().unwrap();

    let end_binding = find_char(&input, 'E');
    let end = end_binding.first().unwrap();

    let result_1 = bfs(start, |p| get_neighbours(&input, *p), |p| *p == *end).unwrap();

    println!("{}", result_1.len() - 1);

    let eligible_stars = find_char(&input, 'a');

    // If you're clever you can just do a single search from the end to any "a"
    // point, but this is cooler and shows off Fearless Concurrency™️
    let result_2 = eligible_stars
        .par_iter()
        .map(|start| bfs(start, |p| get_neighbours(&input, *p), |p| *p == *end))
        .filter_map(|path_res| match path_res {
            Some(path) => Some(path.len()),
            None => None,
        })
        .min()
        .unwrap();

    println!("{}", result_2 - 1);
}
