use std::{
    collections::{HashMap, HashSet},
    io,
};

use itertools::Itertools;

fn antinodes(
    map: &HashMap<char, Vec<(isize, isize)>>,
    map_width: isize,
    map_height: isize,
    handle_resonance: bool,
) -> HashSet<(isize, isize)> {
    let mut output = HashSet::new();

    for (_, positions) in map {
        for pos1 in positions {
            for pos2 in positions {
                if pos1 == pos2 {
                    continue;
                }

                let vector = (pos1.0 - pos2.0, pos1.1 - pos2.1);

                let mut antinode = (pos1.0 + vector.0, pos1.1 + vector.1);
                let mut resonant_antinode = (pos1.0 - vector.0, pos1.1 - vector.1);

                loop {
                    let is_valid_antinode = antinode.0 >= 0
                        && antinode.1 >= 0
                        && antinode.0 < map_height
                        && antinode.1 < map_width
                        && (handle_resonance || !positions.contains(&antinode));

                    if is_valid_antinode {
                        output.insert(antinode);
                    } else if !handle_resonance {
                        break;
                    }

                    if handle_resonance {
                        if resonant_antinode.0 >= 0
                            && resonant_antinode.1 >= 0
                            && resonant_antinode.0 < map_height
                            && resonant_antinode.1 < map_width
                        {
                            output.insert(resonant_antinode);
                        } else {
                            break;
                        }

                        antinode = (antinode.0 + vector.0, antinode.1 + vector.1);
                        resonant_antinode = (
                            resonant_antinode.0 - vector.0,
                            resonant_antinode.1 - vector.1,
                        );
                    } else {
                        break;
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

    let part_one = antinodes(&map, map_width as isize, map_height as isize, false).len();

    println!("{}", part_one);

    let part_two = antinodes(&map, map_width as isize, map_height as isize, true).len();

    println!("{part_two}");
}
