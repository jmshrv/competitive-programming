use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
    iter::repeat,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn expand_input(input: &Vec<String>) -> Vec<String> {
    let mut vertically_expanded_input = input
        .iter()
        .map(|line| {
            if line.contains('#') {
                line.clone()
            } else {
                repeat('!').take(line.chars().count()).collect::<String>()
            }
        })
        // .flatten()
        .collect::<Vec<_>>();

    let vertical_length = vertically_expanded_input
        .first()
        .expect("No first line?")
        .chars()
        .count();

    let mut empty_columns = vec![];

    'columns: for i in 0..vertical_length {
        for line in &vertically_expanded_input {
            let char = line.chars().nth(i).expect("Failed to get column char!");

            // If we see a galaxy, skip this column
            if char == '#' {
                continue 'columns;
            }
        }

        empty_columns.push(i);
    }

    for empty_column in empty_columns {
        for line in &mut vertically_expanded_input {
            let mut chars = line.chars().collect::<Vec<_>>();
            chars[empty_column] = '!';
            *line = chars.iter().collect();
        }
    }

    vertically_expanded_input
}

fn find_galaxies(input: &Vec<String>) -> HashMap<usize, (usize, usize)> {
    let mut galaxies = HashMap::new();

    for i in input.iter().enumerate() {
        for j in i.1.char_indices() {
            if j.1 == '#' {
                galaxies.insert(galaxies.len() + 1, (i.0, j.0));
            }
        }
    }

    galaxies
}

fn next_steps(input: &Vec<String>, index: (usize, usize)) -> Vec<(usize, usize)> {
    let max_0 = input.len() - 1;
    let max_1 = input
        .first()
        .expect("No first line in input?")
        .chars()
        .count()
        - 1;

    let mut steps = vec![];

    if index.0 != 0 {
        steps.push((index.0 - 1, index.1));
    }

    if index.0 != max_0 {
        steps.push((index.0 + 1, index.1));
    }

    if index.1 != 0 {
        steps.push((index.0, index.1 - 1))
    }

    if index.1 != max_1 {
        steps.push((index.0, index.1 + 1));
    }

    steps
}

fn shortest_path_len(
    input: &Vec<String>,
    from: (usize, usize),
    to: (usize, usize),
    expansion_length: usize,
) -> usize {
    let mut vertical_distance = 0;
    let mut horizontal_distance = 0;

    for i in from.0.min(to.0)..from.0.max(to.0) {
        let char = input[i].chars().nth(from.1).expect("Failed to get char!");

        if char == '!' {
            vertical_distance += expansion_length;
        } else {
            vertical_distance += 1;
        }
    }

    for i in from.1.min(to.1)..from.1.max(to.1) {
        let char = input[to.0].chars().nth(i).expect("Failed to get char!");

        if char == '!' {
            horizontal_distance += expansion_length;
        } else {
            horizontal_distance += 1;
        }
    }

    horizontal_distance + vertical_distance
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    let expanded_input = expand_input(&input);

    let galaxies = find_galaxies(&expanded_input);

    // let mut part1_answer = 0;

    let mut pairs = HashSet::new();

    for galaxy_1 in &galaxies {
        for galaxy_2 in &galaxies {
            if galaxy_1.0 == galaxy_2.0 {
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
        .map(|pair| shortest_path_len(&expanded_input, *pair.0 .1, *pair.1 .1, 2))
        .sum();

    println!("{part1_answer}");

    let part2_answer: usize = pairs
        .par_iter()
        .map(|pair| shortest_path_len(&expanded_input, *pair.0 .1, *pair.1 .1, 1000000))
        .sum();

    println!("{part2_answer}");
}
