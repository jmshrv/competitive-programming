use std::{io, iter};

use either::Either;
use itertools::Itertools;

fn debug_print(drive: &[Option<usize>]) {
    let drive_str = drive
        .iter()
        .map(|block| match block {
            Some(block) => char::from_digit(*block as u32, 10).unwrap(),
            None => '.',
        })
        .collect::<String>();

    println!("{drive_str}");
}

fn defrag(drive: &mut [Option<usize>]) {
    while drive
        .iter()
        .chunk_by(|block| block.is_some())
        .into_iter()
        .count()
        > 2
    {
        let first_free = drive.iter().position(|block| block.is_none());
        let last_chunk = drive.iter().rposition(|block| block.is_some());

        if let (Some(first_free), Some(last_chunk)) = (first_free, last_chunk) {
            drive.swap(first_free, last_chunk);
        }
    }
}

fn defrag_full(drive: &[Option<usize>]) -> Vec<Option<usize>> {
    let mut drive_copy = drive.to_vec();
    let mut last_processed_chunk = usize::MAX;

    while last_processed_chunk != 0 {
        let random_clone_to_make_borrow_happy = drive_copy.clone();

        let chunked_drive = random_clone_to_make_borrow_happy
            .iter()
            .enumerate()
            .chunk_by(|(_, block)| **block)
            .into_iter()
            .map(|(is_some, chunk)| (is_some, chunk.collect_vec()))
            .collect_vec();

        let last_chunk_opt = chunked_drive
            .iter()
            .filter(|(block, _)| block.is_some())
            .rfind(|(block, _)| block.unwrap() < last_processed_chunk);

        if last_chunk_opt.is_none() {
            return drive_copy;
        }

        let last_chunk = &last_chunk_opt.unwrap().1;

        last_processed_chunk = last_chunk[0].1.unwrap();

        let last_chunk_size = last_chunk.len();

        let fitting_empty_chunk = chunked_drive
            .iter()
            .filter(|(block, _)| block.is_none())
            .find(|(_, chunk)| chunk.len() >= last_chunk_size && chunk[0].0 < last_chunk[0].0);

        if let Some((_, fitting_empty_chunk)) = fitting_empty_chunk {
            let chunk_id = last_chunk[0].1;

            let last_chunk_range = last_chunk[0].0..=last_chunk.last().unwrap().0;

            for i in last_chunk_range {
                drive_copy[i] = None;
            }

            let empty_range_start = fitting_empty_chunk[0].0;

            for block in drive_copy
                .iter_mut()
                .skip(empty_range_start)
                .take(last_chunk_size)
            {
                *block = *chunk_id;
            }
        }
    }

    drive_copy
}

fn checksum(drive: &[Option<usize>]) -> usize {
    drive
        .iter()
        .map(|block| block.unwrap_or(0))
        .enumerate()
        .map(|(index, block)| index * block)
        .sum()
}

fn expand_chunk(index: usize, chunk: &[usize]) -> impl Iterator<Item = Option<usize>> {
    match chunk {
        [chunk_size, free_size] => Either::Left(
            iter::repeat_n(Some(index), *chunk_size).chain(iter::repeat_n(None, *free_size)),
        ),
        [chunk_size] => Either::Right(iter::repeat_n(Some(index), *chunk_size)),
        _ => panic!("Invalid chunk!"),
    }
}

fn main() {
    let input = io::read_to_string(io::stdin())
        .unwrap()
        .chars()
        .map(|char| char.to_digit(10).unwrap() as usize)
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(|(index, chunk)| expand_chunk(index, &chunk.collect_vec()))
        .collect_vec();

    let mut part_one_drive = input.clone();
    defrag(&mut part_one_drive);

    let part_one = checksum(&part_one_drive);

    println!("{part_one}");

    let part_two_drive = defrag_full(&input);

    let part_two = checksum(&part_two_drive);
    println!("{part_two}");
}
