use std::{collections::HashMap, io};

use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

fn move_up(input: &mut Vec<char>, column_index: usize) {
    // let mut recurse = false;

    // if column_index == input.len() - 1 {
    //     return;
    // }

    // // if let Some(row) = input.get_mut(row_index - 1) {
    // if let Some(cell) = input.get_mut(column_index + 1) {
    //     if *cell == '.' {
    //         *cell = 'O';
    //         recurse = true;
    //     }
    // }
    // // }

    // if recurse && column_index != input.len() - 1 {
    //     *input.get_mut(column_index).unwrap() = '.';

    //     move_up(input, column_index + 1);
    // }

    let mut collected = 0;
    let mut stop = input.len();

    for i in column_index..input.len() {
        if input[i] == 'O' {
            input[i] = '.';
            collected += 1;
        } else if input[i] == '#' {
            stop = i;
            break;
        }
    }

    for i in (stop - collected)..stop {
        input[i] = 'O';
    }
}

fn shift_north(input: &mut Vec<Vec<char>>, row_cache: &mut HashMap<Vec<char>, Vec<char>>) {
    let original = input.clone();

    let mut rotated_input = rotate_input(&input);

    // for line in &rotated_input {
    //     println!("{}", line.iter().collect::<String>());
    // }

    // println!();

    // let column_length = input.first().expect("No lines!").len();

    rotated_input.iter_mut().for_each(|row| {
        if let Some(cached) = row_cache.get(row) {
            *row = cached.clone();
        } else {
            let original_row = row.clone();

            for i in 0..row.len() {
                if row[i] == 'O' {
                    move_up(row, i);
                }
            }

            row_cache.insert(original_row, row.clone());
        }
    });

    // for line in &rotated_input {
    //     println!("{}", line.iter().collect::<String>());
    // }

    // println!();

    // for column_index in 0..column_length {
    //     // // iter_mut doesn't support windows :(
    //     // for (bottom_index, top_index) in (input.len()..0).tuple_windows() {
    //     //     let bottom = input
    //     //         .get_mut(bottom_index)
    //     //         .expect("Failed to get bottom row!");

    //     //     let top = input.get_mut(top_index).expect("Failed to get top row!");

    //     //     if bottom[column_index] == 'O' {

    //     //     }
    //     // }

    //     // let column_before = input
    //     //     .iter()
    //     //     // .enumerate()
    //     //     .map(|line| line[column_index])
    //     //     // .filter(|(_, char)| *char != '.')
    //     //     .collect::<Vec<_>>();

    //     // if let Some(cached) = cache.get(&column_before) {
    //     //     // println!("cache hit!");

    //     //     input
    //     //         .iter_mut()
    //     //         .zip(cached)
    //     //         .for_each(|(line, cached_char)| line[column_index] = *cached_char);

    //     //     continue;
    //     // }

    //     for row_index in 0..input.len() {
    //         let cell = input
    //             .get_mut(row_index)
    //             .expect("Failed to get row!")
    //             .get_mut(column_index)
    //             .expect("Failed to get column!");

    //         if *cell == 'O' {
    //             move_up(input, row_index, column_index);
    //         }
    //     }

    //     // let column = input
    //     //     .iter()
    //     //     .map(|line| line[column_index])
    //     //     .collect::<Vec<_>>();

    //     // cache.insert(column_before, column);
    // }

    let upside_down_input = rotate_input(&rotated_input);
    let nearly_there_input = rotate_input(&upside_down_input);
    let unrotated_input = rotate_input(&nearly_there_input);

    *input = unrotated_input.clone();
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

    let mut row_cache = HashMap::new();

    // for line in &part1_input {
    //     println!("{}", line.iter().collect::<String>());
    // }

    // println!();

    shift_north(&mut part1_input, &mut row_cache);

    // for line in &part1_input {
    //     println!("{}", line.iter().collect::<String>());
    // }

    // println!();

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

    let progress_bar = ProgressBar::new(1000000000)
        .with_style(ProgressStyle::with_template("{wide_bar} {pos}/{len} {eta_precise}").unwrap());

    let mut history = HashMap::new();

    for i in 0..1000000000 {
        for _ in 0..4 {
            shift_north(&mut part2_input, &mut row_cache);
            part2_input = rotate_input(&part2_input);
        }

        if let Some(previous) = history.insert(part2_input.clone(), i) {
            let steps = i - previous;
            let remaining = 1000000000 - i;
            for _ in 0..(remaining % steps) - 1 {
                for _ in 0..4 {
                    shift_north(&mut part2_input, &mut row_cache);
                    part2_input = rotate_input(&part2_input);
                }
            }
            break;
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
