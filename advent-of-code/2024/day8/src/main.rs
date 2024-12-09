use std::{
    collections::{HashMap, HashSet},
    io,
};

use itertools::Itertools;

fn antinodes(
    map: &HashMap<char, Vec<(isize, isize)>>,
    map_width: isize,
    map_height: isize,
) -> HashSet<(isize, isize)> {
    let mut output = HashSet::new();

    for (_, positions) in map {
        for pos1 in positions {
            for pos2 in positions {
                if pos1 == pos2 {
                    continue;
                }

                let vector = (
                    pos1.0.abs_diff(pos2.0) as isize,
                    pos1.1.abs_diff(pos2.1) as isize,
                );

                if pos1.0 - vector.0 >= 0 && pos1.1 - vector.1 >= 0 {
                    let antinode = (pos1.0 - vector.0, pos1.1 - vector.1);

                    if antinode != *pos2 {
                        output.insert((pos1.0 - vector.0, pos1.1 - vector.1));
                    }
                }

                if pos1.0 + vector.0 < map_height && pos1.1 + vector.1 < map_width {
                    let antinode = (pos1.0 + vector.0, pos1.1 + vector.1);

                    if antinode != *pos2 {
                        output.insert(antinode);
                    }
                }
            }
        }
    }

    output
}

fn debug_print(input: &[String], antinodes: &HashSet<(isize, isize)>) {
    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.char_indices() {
            if antinodes.contains(&(y as isize, x as isize)) {
                print!("#");
            } else {
                print!("{cell}");
            }
        }

        println!()
    }
}

fn main() {
    let input = io::stdin().lines().map(Result::unwrap).collect::<Vec<_>>();

    let map_height = input.len();
    let map_width = input.first().unwrap().len();

    let map = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, char)| *char != '.')
                .map(move |(x, char)| (char, (y as isize, x as isize)))
        })
        .into_group_map();

    let part_one = antinodes(&map, map_width as isize, map_height as isize);

    debug_print(&input, &part_one);

    println!("{}", part_one.len());
}
