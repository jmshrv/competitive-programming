use std::io;

use itertools::Itertools;

fn horizontal_reflection(pattern: &[String], fix_smudge: bool) -> Option<usize> {
    let reflections = if fix_smudge {
        pattern
            .windows(2)
            .positions(|pair| {
                pair[0]
                    .chars()
                    .zip(pair[1].chars())
                    .filter(|(top_c, bottom_c)| top_c != bottom_c)
                    .count()
                    <= 1
            })
            .collect::<Vec<_>>()
    } else {
        pattern
            .windows(2)
            .positions(|pair| pair[0] == pair[1])
            .collect::<Vec<_>>()
    };

    'reflections: for reflection_index in reflections {
        let mut delta = 1;
        let mut diffs = 0;

        if pattern[reflection_index] != pattern[reflection_index + 1] {
            diffs += 1;
        }

        while let (Some(top), Some(bottom)) = (
            pattern.get(reflection_index.overflowing_sub(delta).0), // not needed for release
            pattern.get(reflection_index + delta + 1),
        ) {
            if top == bottom {
                delta += 1;
            } else if fix_smudge {
                diffs += 1;
                delta += 1;
            } else {
                continue 'reflections;
            }
        }

        if !fix_smudge {
            return Some(reflection_index);
        }

        if fix_smudge && diffs == 1 {
            return Some(reflection_index);
        }
    }

    None
}

fn vertical_reflection(pattern: &[String], fix_smudge: bool) -> Option<usize> {
    let line_length = pattern
        .first()
        .expect("Failed to get first pattern line!")
        .chars()
        .count();

    let mut rotated_pattern = vec![];

    for column_index in 0..line_length {
        let column = pattern
            .iter()
            .map(|line| {
                line.chars()
                    .nth(column_index)
                    .expect("Failed to get column char!")
            })
            .collect::<String>();

        rotated_pattern.push(column);
    }

    horizontal_reflection(&rotated_pattern, fix_smudge)
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    let mut part1_answer = 0;

    for pattern in input.split(|line| line.is_empty()) {
        if let Some(index) = horizontal_reflection(pattern, false) {
            part1_answer += (index + 1) * 100;
        } else if let Some(index) = vertical_reflection(pattern, false) {
            part1_answer += index + 1;
        } else {
            panic!("Failed to find reflection!");
        }
    }

    println!("{part1_answer}");

    let mut part2_answer = 0;

    for pattern in input.split(|line| line.is_empty()) {
        if let Some(index) = horizontal_reflection(pattern, true) {
            part2_answer += (index + 1) * 100;
        } else if let Some(index) = vertical_reflection(pattern, true) {
            part2_answer += index + 1;
        } else {
            panic!("Failed to find reflection!");
        }
    }

    println!("{part2_answer}");
}
