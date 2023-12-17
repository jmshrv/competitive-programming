use std::{
    collections::{HashSet, VecDeque},
    io,
};

fn reflect_forward_slash(direction_x: isize, direction_y: isize) -> (isize, isize) {
    let new_direction_x = match direction_x {
        -1 => 0,
        0 => match direction_y {
            -1 => 1,
            0 => panic!("direction_x and direction_y can't both be zero!"),
            1 => -1,
            _ => panic!("Invalid direction_y {direction_y}!"),
        },
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
        // 0 => match direction_y {
        //     -1 => -1,
        //     0 => panic!("direction_x and direction_y can't both be zero!"),
        //     1 => -1,
        // },
        0 => direction_y,
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
    start_x: isize,
    start_y: isize,
    start_direction_x: isize,
    start_direction_y: isize,
) -> HashSet<((isize, isize), (isize, isize))> {
    let mut explored = HashSet::new();
    let mut queue = VecDeque::new();

    explored.insert(((start_x, start_y), (start_direction_x, start_direction_y)));
    queue.push_back(((start_x, start_y), (start_direction_x, start_direction_y)));

    while let Some(((x, y), (direction_x, direction_y))) = queue.pop_front() {
        if let Some(cell) = input.get(y as usize).and_then(|row| row.get(x as usize)) {
            // let ((next_x, next_y), (next_direction_x, next_direction_y)) = match cell {
            let nexts = match cell {
                '.' => vec![(
                    (x + direction_x, y + direction_y),
                    (direction_x, direction_y),
                )],
                '/' => {
                    let (new_direction_x, new_direction_y) =
                        reflect_forward_slash(direction_x, direction_y);

                    vec![(
                        (x + new_direction_x, y + new_direction_y),
                        (new_direction_x, new_direction_y),
                    )]
                }
                '\\' => {
                    let (new_direction_x, new_direction_y) =
                        reflect_backslash(direction_x, direction_y);

                    vec![(
                        (x + new_direction_x, y + new_direction_y),
                        (new_direction_x, new_direction_y),
                    )]
                }
                '|' => {
                    if direction_x == 0 {
                        vec![(
                            (x + direction_x, y + direction_y),
                            (direction_x, direction_y),
                        )]
                    } else {
                        vec![((x, y - 1), (0, -1)), ((x, y + 1), (0, 1))]
                    }
                }
                '-' => {
                    if direction_y == 0 {
                        vec![(
                            (x + direction_x, y + direction_y),
                            (direction_x, direction_y),
                        )]
                    } else {
                        vec![((x - 1, y), (-1, 0)), ((x + 1, y), (1, 0))]
                    }
                }
                _ => panic!("Unknown cell {}!", cell),
            };

            for next in nexts {
                if input
                    .get(next.0 .1 as usize)
                    .and_then(|row| row.get(next.0 .0 as usize))
                    .is_some()
                {
                    if explored.insert(next) {
                        queue.push_back(next);
                    }
                }
            }
        }
    }

    explored
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let visited = charged(&input, 0, 0, 1, 0);
    let visited_pos_only = visited.iter().map(|(x_y, _)| x_y).collect::<HashSet<_>>();

    let part1_answer = visited_pos_only.len();
    println!("{part1_answer}");

    let y_length = input.len() as isize;
    let x_length = input.first().expect("No first?").len() as isize;

    let mut part2_answer = 0;

    for y in 0..y_length {
        let left = charged(&input, 0, y, 1, 0);
        let left_visited_only = left.iter().map(|(x_y, _)| x_y).collect::<HashSet<_>>();

        let right = charged(&input, x_length - 1, y, -1, 0);
        let right_visited_only = right.iter().map(|(x_y, _)| x_y).collect::<HashSet<_>>();

        let max = left_visited_only.len().max(right_visited_only.len());

        if part2_answer < max {
            part2_answer = max;
        }
    }

    for x in 0..x_length {
        let down = charged(&input, x, 0, 0, 1);
        let down_visited_only = down.iter().map(|(x_y, _)| x_y).collect::<HashSet<_>>();

        let up = charged(&input, x, y_length - 1, 0, -1);
        let up_visited_only = up.iter().map(|(x_y, _)| x_y).collect::<HashSet<_>>();

        let max = up_visited_only.len().max(down_visited_only.len());

        if part2_answer < max {
            part2_answer = max;
        }
    }

    println!("{part2_answer}");
}
