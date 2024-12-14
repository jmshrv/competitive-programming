use std::{collections::HashSet, io};

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: (i64, i64),
    vector: (i64, i64),
}

impl Robot {
    fn step(&mut self, map_x: i64, map_y: i64) -> Robot {
        let new_pos = (
            (self.position.0 + self.vector.0).rem_euclid(map_x),
            (self.position.1 + self.vector.1).rem_euclid(map_y),
        );

        // while new_pos.0 < 0 {
        //     new_pos.0 = map_x - new_pos.0;
        // }

        // while new_pos.1 < 0 {
        //     new_pos.1 = map_y - new_pos.1;
        // }

        // let modded_pos = ((new_pos.0 % map_x), (new_pos.1 % map_y));

        Robot {
            position: new_pos,
            vector: self.vector,
        }
    }
}

fn parse_robot(line: &str) -> Option<Robot> {
    let (pos_str, vec_str) = line.split_once(' ')?;

    let (pos_x_str, pos_y_str) = pos_str.strip_prefix("p=")?.split_once(',')?;
    let (vec_x_str, vec_y_str) = vec_str.strip_prefix("v=")?.split_once(',')?;

    let pos_x = pos_x_str.parse::<i64>().ok()?;
    let pos_y = pos_y_str.parse::<i64>().ok()?;
    let vec_x = vec_x_str.parse::<i64>().ok()?;
    let vec_y = vec_y_str.parse::<i64>().ok()?;

    Some(Robot {
        position: (pos_x, pos_y),
        vector: (vec_x, vec_y),
    })
}

fn debug_print(robots: &[Robot], map_width: i64, map_height: i64) {
    let positions = robots
        .iter()
        .map(|robot| robot.position)
        .collect::<HashSet<_>>();

    for y in 0..map_height {
        for x in 0..map_width {
            if positions.contains(&(x, y)) {
                print!("B");
            } else {
                print!(".")
            }
        }
        println!();
    }
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| parse_robot(&line).unwrap())
        .collect::<Vec<_>>();

    let map_width = 101;
    let map_height = 103;

    let part_one_robots = input
        .iter()
        .map(|robot| {
            let mut new_robot = *robot;

            for _ in 0..100 {
                new_robot = new_robot.step(map_width, map_height);
            }

            new_robot
        })
        .collect::<Vec<_>>();

    let part_one_top_left = part_one_robots
        .iter()
        .filter(|robot| robot.position.0 < map_width / 2 && robot.position.1 < map_height / 2)
        .count();

    let part_one_bottom_left = part_one_robots
        .iter()
        .filter(|robot| robot.position.0 < map_width / 2 && robot.position.1 > map_height / 2)
        .count();

    let part_one_top_right = part_one_robots
        .iter()
        .filter(|robot| robot.position.0 > map_width / 2 && robot.position.1 < map_height / 2)
        .count();

    let part_one_bottom_right = part_one_robots
        .iter()
        .filter(|robot| robot.position.0 > map_width / 2 && robot.position.1 > map_height / 2)
        .count();

    let part_one =
        part_one_bottom_left * part_one_bottom_right * part_one_top_left * part_one_top_right;

    println!("{part_one}");

    let mut working_copy = input.clone();
    let mut seconds = 0;

    loop {
        working_copy
            .iter_mut()
            .for_each(|robot| *robot = robot.step(map_width, map_height));

        seconds += 1;

        let working_copy_map = working_copy
            .iter()
            .map(|robot| robot.position)
            .collect::<HashSet<_>>();

        if working_copy_map.iter().any(|pos| {
            working_copy_map.contains(&(pos.0 + 1, pos.1))
                && working_copy_map.contains(&(pos.0 + 2, pos.1))
                && working_copy_map.contains(&(pos.0 + 3, pos.1))
                && working_copy_map.contains(&(pos.0 + 4, pos.1))
                && working_copy_map.contains(&(pos.0 + 5, pos.1))
                && working_copy_map.contains(&(pos.0 + 6, pos.1))
                && working_copy_map.contains(&(pos.0 + 7, pos.1))
                && working_copy_map.contains(&(pos.0 + 8, pos.1))
                && working_copy_map.contains(&(pos.0 + 9, pos.1))
                && working_copy_map.contains(&(pos.0 + 10, pos.1))
        }) {
            println!("{seconds}");
            return;
        }
    }
}
