use std::{collections::HashSet, io};

fn reflect_forward_slash(direction_x: isize, direction_y: isize) -> (isize, isize) {
    let new_direction_x = match direction_x {
        -1 => 0,
        0 => 1,
        1 => 0,
        _ => panic!("Invalid direction_x {direction_x}!"),
    };

    let new_direction_y = match direction_y {
        -1 => 0,
        0 => match direction_x {
            -1 => 1,
            0 => panic!("direction_x and direction_y can't both be zero!"),
            1 => -1,
            _ => panic!("Invalid direction_x {direction_x}!"),
        },
        1 => 0,
        _ => panic!("Invalid direction_y {direction_y}!"),
    };

    (new_direction_x, new_direction_y)
}

fn reflect_backslash(direction_x: isize, direction_y: isize) -> (isize, isize) {
    let new_direction_x = match direction_x {
        -1 => 0,
        0 => -1,
        1 => 0,
        _ => panic!("Invalid direction_x {direction_x}!"),
    };

    let new_direction_y = match direction_y {
        -1 => 0,
        0 => match direction_x {
            -1 => -1,
            0 => panic!("direction_x and direction_y can't both be zero!"),
            1 => 1,
            _ => panic!("Invalid direction_x {direction_x}!"),
        },
        1 => 0,
        _ => panic!("Invalid direction_y {direction_y}!"),
    };

    (new_direction_x, new_direction_y)
}

fn charged(
    input: &Vec<Vec<char>>,
    mut x: isize,
    mut y: isize,
    mut direction_x: isize,
    mut direction_y: isize,
) -> HashSet<(isize, isize)> {
    let mut charged_cells = HashSet::new();

    while let Some(cell) = input.get(y as usize).and_then(|row| row.get(x as usize)) {
        println!("({x}, {y})");

        if (x, y) == (4, 7) {
            println!("hi!");
        }

        match cell {
            '.' => {}
            '/' => (direction_x, direction_y) = reflect_forward_slash(direction_x, direction_y),
            '\\' => (direction_x, direction_y) = reflect_backslash(direction_x, direction_y),
            '|' => {
                if direction_x != 0 && has_seen_unique {
                    (direction_x, direction_y) = (0, -1);

                    let extra_path = charged(input, x + direction_x, y, 0, 1);

                    charged_cells.extend(extra_path);
                }
            }
            '-' => {
                if direction_y != 0 && has_seen_unique {
                    (direction_x, direction_y) = (-1, 0);

                    charged_cells.extend(charged(input, x, y + direction_y, 1, 0).iter())
                }
            }
            _ => panic!("Unknown cell {}!", cell),
        };

        x += direction_x;
        y += direction_y;
    }

    charged_cells
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let visited = charged(&input, 0, 0, 1, 0);

    for y in 0..input.len() as isize {
        print!("{y}");
        for x in 0..input.first().unwrap().len() as isize {
            if visited.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    let part1_answer = visited.len();

    println!("{part1_answer}");
}
