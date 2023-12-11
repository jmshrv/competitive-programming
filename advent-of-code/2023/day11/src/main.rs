use std::{collections::HashSet, io};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn expand_input(input: &Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let empty_row_indices = input
        .iter()
        .enumerate()
        .filter(|line| !line.1.contains('#'))
        .map(|line| line.0)
        .collect::<Vec<_>>();

    let vertical_length = input.first().expect("No first line?").chars().count();

    let mut empty_column_indices = vec![];

    'columns: for i in 0..vertical_length {
        for line in input {
            let char = line.chars().nth(i).expect("Failed to get column char!");

            // If we see a galaxy, skip this column
            if char == '#' {
                continue 'columns;
            }
        }

        empty_column_indices.push(i);
    }

    (empty_row_indices, empty_column_indices)

    // for empty_column in empty_columns {
    //     for line in &mut vertically_expanded_input {
    //         let mut chars = line.chars().collect::<Vec<_>>();
    //         chars[empty_column] = '!';
    //         *line = chars.iter().collect();
    //     }
    // }

    // vertically_expanded_input
}

fn find_galaxies(input: &Vec<String>) -> HashSet<(usize, usize)> {
    let mut galaxies = HashSet::new();

    for i in input.iter().enumerate() {
        for j in i.1.char_indices() {
            if j.1 == '#' {
                galaxies.insert((i.0, j.0));
            }
        }
    }

    galaxies
}

fn shortest_path_len(
    empty_lines: &(Vec<usize>, Vec<usize>),
    from: (usize, usize),
    to: (usize, usize),
    expansion_length: usize,
) -> usize {
    let uncorrected_horizontal_distance = from.1.max(to.1) - from.1.min(to.1);
    let uncorrected_vertical_distance = from.0.max(to.0) - from.0.min(to.0);

    // Weirdly, making empty_lines hashsets actually makes stuff slower?
    let expanded_rows_travelled = (from.0.min(to.0)..from.0.max(to.0))
        .filter(|row_index| empty_lines.0.contains(&row_index))
        .count();

    let expanded_columns_travelled = (from.1.min(to.1)..from.1.max(to.1))
        .filter(|column_index| empty_lines.1.contains(&column_index))
        .count();

    let horizontal_distance = (uncorrected_horizontal_distance - expanded_columns_travelled)
        + (expanded_columns_travelled * expansion_length);

    let vertical_distance = (uncorrected_vertical_distance - expanded_rows_travelled)
        + (expanded_rows_travelled * expansion_length);

    horizontal_distance + vertical_distance
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    let empty_lines = expand_input(&input);

    let galaxies = find_galaxies(&input);

    // let mut part1_answer = 0;

    let mut pairs = HashSet::new();

    for galaxy_1 in &galaxies {
        for galaxy_2 in &galaxies {
            if galaxy_1 == galaxy_2 {
                continue;
            }

            if pairs.contains(&(galaxy_2, galaxy_1)) {
                continue;
            }

            pairs.insert((galaxy_1, galaxy_2));

            // part1_answer += shortest_path_len(*galaxy_1.1, *galaxy_2.1);
        }
    }

    let part1_answer: usize = pairs
        .par_iter()
        .map(|pair| shortest_path_len(&empty_lines, *pair.0, *pair.1, 2))
        .sum();

    println!("{part1_answer}");

    let part2_answer: usize = pairs
        .par_iter()
        .map(|pair| shortest_path_len(&empty_lines, *pair.0, *pair.1, 1000000))
        .sum();

    println!("{part2_answer}");
}
