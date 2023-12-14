use std::{collections::HashMap, io};

use indicatif::ProgressBar;

fn move_up(input: &mut Vec<Vec<char>>, row_index: usize, column_index: usize) {
    let mut recurse = false;

    if row_index == 0 {
        return;
    }

    if let Some(row) = input.get_mut(row_index - 1) {
        if let Some(cell) = row.get_mut(column_index) {
            if *cell == '.' {
                *cell = 'O';
                recurse = true;
            }
        }
    }

    if recurse && row_index != 0 {
        *input
            .get_mut(row_index)
            .unwrap()
            .get_mut(column_index)
            .unwrap() = '.';

        move_up(input, row_index - 1, column_index);
    } else {
    }
}

fn shift_north(input: &mut Vec<Vec<char>>, cache: &mut HashMap<Vec<char>, Vec<char>>) {
    let column_length = input.first().expect("No lines!").len();

    for column_index in 0..column_length {
        // // iter_mut doesn't support windows :(
        // for (bottom_index, top_index) in (input.len()..0).tuple_windows() {
        //     let bottom = input
        //         .get_mut(bottom_index)
        //         .expect("Failed to get bottom row!");

        //     let top = input.get_mut(top_index).expect("Failed to get top row!");

        //     if bottom[column_index] == 'O' {

        //     }
        // }

        let column_before = input
            .iter()
            // .enumerate()
            .map(|line| line[column_index])
            // .filter(|(_, char)| *char != '.')
            .collect::<Vec<_>>();

        if let Some(cached) = cache.get(&column_before) {
            // println!("cache hit!");

            input
                .iter_mut()
                .zip(cached)
                .for_each(|(line, cached_char)| line[column_index] = *cached_char);

            continue;
        }

        for row_index in 0..input.len() {
            let cell = input
                .get_mut(row_index)
                .expect("Failed to get row!")
                .get_mut(column_index)
                .expect("Failed to get column!");

            if *cell == 'O' {
                move_up(input, row_index, column_index);
            }
        }

        let column = input
            .iter()
            .map(|line| line[column_index])
            .collect::<Vec<_>>();

        cache.insert(column_before, column);
    }
}

fn rotate_input(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let line_length = input
        .first()
        .expect("Failed to get first pattern line!")
        .len();

    let mut rotated_input = vec![];

    for column_index in 0..line_length {
        let column = input
            .iter()
            .map(|line| line[column_index])
            .rev()
            .collect::<Vec<_>>();

        rotated_input.push(column);
    }

    rotated_input
}

fn main() {
    let mut part1_input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut part2_input = part1_input.clone();

    let mut cache = HashMap::new();

    // for line in &input {
    //     println!("{}", line.iter().collect::<String>());
    // }

    // println!();

    shift_north(&mut part1_input, &mut cache);

    let part1_answer: usize = part1_input
        .iter()
        .enumerate()
        .map(|line| line.1.iter().filter(|c| **c == 'O').count() * (part1_input.len() - line.0))
        .sum();

    println!("{part1_answer}");

    // for _ in 0..3 {
    //     for _ in 0..4 {
    //         shift_north(&mut part2_input);
    //         part2_input = rotate_input(&part2_input);
    //     }

    //     for line in &part2_input {
    //         println!("{}", line.iter().collect::<String>());
    //     }
    //     println!();
    // }

    let progress_bar = ProgressBar::new(1000000000);

    for i in 0..1000000000 {
        for _ in 0..4 {
            shift_north(&mut part2_input, &mut cache);
            part2_input = rotate_input(&part2_input);
        }

        if i % 10000 == 0 {
            progress_bar.inc(10000);
        }
    }

    let part2_answer: usize = part2_input
        .iter()
        .enumerate()
        .map(|line| line.1.iter().filter(|c| **c == 'O').count() * (part2_input.len() - line.0))
        .sum();

    println!("{part2_answer}");
}
