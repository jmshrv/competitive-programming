use std::io;

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

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| parse_robot(&line).unwrap())
        .collect::<Vec<_>>();

    let part_one_map_width = 101;
    let part_one_map_height = 103;

    let part_one_robots = input
        .iter()
        .map(|robot| {
            let mut new_robot = *robot;

            for _ in 0..100 {
                new_robot = new_robot.step(part_one_map_width, part_one_map_height);
            }

            new_robot
        })
        .collect::<Vec<_>>();

    let part_one_top_left = part_one_robots
        .iter()
        .filter(|robot| {
            robot.position.0 < part_one_map_width / 2 && robot.position.1 < part_one_map_height / 2
        })
        .count();

    let part_one_bottom_left = part_one_robots
        .iter()
        .filter(|robot| {
            robot.position.0 < part_one_map_width / 2 && robot.position.1 > part_one_map_height / 2
        })
        .count();

    let part_one_top_right = part_one_robots
        .iter()
        .filter(|robot| {
            robot.position.0 > part_one_map_width / 2 && robot.position.1 < part_one_map_height / 2
        })
        .count();

    let part_one_bottom_right = part_one_robots
        .iter()
        .filter(|robot| {
            robot.position.0 > part_one_map_width / 2 && robot.position.1 > part_one_map_height / 2
        })
        .count();

    let part_one =
        part_one_bottom_left * part_one_bottom_right * part_one_top_left * part_one_top_right;

    println!("{part_one}");
}
