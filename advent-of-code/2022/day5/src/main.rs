#![warn(clippy::pedantic)]

#[macro_use]
extern crate scan_fmt;

use std::{collections::VecDeque, io};

fn filter_stack(stack_lines: &[String]) -> Vec<Vec<char>> {
    let mut out = Vec::new();

    for line in stack_lines {
        let line_chars: Vec<_> = line.chars().collect();
        let mut line_out = Vec::new();

        for i in (1..line_chars.len()).step_by(4) {
            line_out.push(line_chars[i]);
        }

        out.push(line_out);
    }

    out
}

fn main() {
    let input: Vec<_> = io::stdin()
        .lines()
        .map(|line_res| line_res.unwrap())
        .collect();

    let (stack_lines_with_index, instructions) =
        input.split_at(input.iter().position(|line| line.is_empty()).unwrap());
    let (stack_index, stack_lines) = stack_lines_with_index.split_last().unwrap();

    let stack_count = stack_index
        .chars()
        .filter(|char| *char != ' ')
        .last()
        .map(|char| char.to_digit(10).unwrap())
        .unwrap();

    let stack_lines_filtered = filter_stack(stack_lines);

    let mut stacks_part_1: Vec<VecDeque<char>> =
        (0..stack_count).map(|_| VecDeque::new()).collect();

    for stack_line in stack_lines_filtered {
        for i in 0..stack_line.len() {
            let item = stack_line[i];

            if item == ' ' {
                continue;
            }

            stacks_part_1[i].push_front(item);
        }
    }

    let mut stacks_part_2 = stacks_part_1.clone();

    let mut skipped_first = false;
    for instruction in instructions {
        if !skipped_first {
            skipped_first = true;
            continue;
        }

        let (amount, src, dest) =
            scan_fmt!(instruction, "move {d} from {d} to {d}", usize, usize, usize).unwrap();

        for _ in 0..amount {
            let taken = stacks_part_1[src - 1].pop_back().unwrap();
            stacks_part_1[dest - 1].push_back(taken);
        }
    }

    for stack in stacks_part_1 {
        print!("{}", stack.back().unwrap());
    }

    println!();

    skipped_first = false;

    for instruction in instructions {
        if !skipped_first {
            skipped_first = true;
            continue;
        }

        let (amount, src, dest) =
            scan_fmt!(instruction, "move {d} from {d} to {d}", usize, usize, usize).unwrap();

        let mut src_stack = VecDeque::new();

        for _ in 0..amount {
            src_stack.push_front(stacks_part_2[src - 1].pop_back().unwrap());
        }

        stacks_part_2[dest - 1].append(&mut src_stack);
    }

    for stack in stacks_part_2 {
        print!("{}", stack.back().unwrap());
    }

    println!();
}
