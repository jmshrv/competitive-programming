use std::{collections::HashSet, io};

fn pos_diff(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    (head_pos.0 - tail_pos.0, head_pos.1 - tail_pos.1)
}

fn is_touching(head_pos: (i32, i32), tail_pos: (i32, i32)) -> bool {
    let (x_diff, y_diff) = pos_diff(head_pos, tail_pos);

    (x_diff <= 1 && x_diff >= -1) && (y_diff <= 1 && y_diff >= -1)
}

fn move_tail(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    debug_assert_eq!(is_touching(head_pos, tail_pos), false);

    let (x_diff, y_diff) = pos_diff(head_pos, tail_pos);
    return (tail_pos.0 + x_diff.signum(), tail_pos.1 + y_diff.signum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_touching_true() {
        assert_eq!(is_touching((1, 1), (1, 1)), true); // Intersect
        assert_eq!(is_touching((1, 0), (0, 0)), true); // Head right
        assert_eq!(is_touching((0, 0), (1, 0)), true); // Head left
        assert_eq!(is_touching((0, 1), (0, 0)), true); // Head up
        assert_eq!(is_touching((0, 0), (0, 1)), true); // Head down
        assert_eq!(is_touching((0, 0), (1, 1)), true); // Head bottom left
        assert_eq!(is_touching((2, 0), (1, 1)), true); // Head bottom right
        assert_eq!(is_touching((0, 1), (1, 0)), true); // Head top left
        assert_eq!(is_touching((1, 1), (0, 0)), true); // Head top right
    }

    #[test]
    fn test_is_touching_false() {
        assert_eq!(is_touching((2, 0), (0, 0)), false); // Head right
        assert_eq!(is_touching((0, 0), (2, 0)), false); // Head left
        assert_eq!(is_touching((0, 2), (0, 0)), false); // Head up
        assert_eq!(is_touching((0, 0), (0, 2)), false); // Head down
        assert_eq!(is_touching((0, 0), (2, 1)), false); // Head bottom left
        assert_eq!(is_touching((3, 0), (1, 1)), false); // Head bottom right
        assert_eq!(is_touching((0, 2), (1, 0)), false); // Head top left
        assert_eq!(is_touching((2, 1), (0, 0)), false); // Head top right
    }
}

fn main() {
    let input: Vec<(char, usize)> = io::stdin()
        .lines()
        .map(|line_res| {
            let line = line_res.unwrap();
            let split = line.split_once(' ').unwrap();
            let direction = split.0.chars().next().unwrap();
            (direction, split.1.parse().unwrap())
        })
        .collect();

    let mut part1_head_pos = (0, 0);
    let mut part1_tail_pos = (0, 0);

    let mut part1_visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    for line in &input {
        for _ in 0..line.1 {
            match line.0 {
                'U' => part1_head_pos.1 += 1,
                'D' => part1_head_pos.1 -= 1,
                'L' => part1_head_pos.0 -= 1,
                'R' => part1_head_pos.0 += 1,
                _ => panic!("Unexpected direction {}", line.0),
            }

            if !is_touching(part1_head_pos, part1_tail_pos) {
                part1_tail_pos = move_tail(part1_head_pos, part1_tail_pos);
                part1_visited.insert(part1_tail_pos);
            }
        }
    }

    println!("{}", part1_visited.len());

    let mut ropes = [
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];

    let mut part2_visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    for line in input {
        for _ in 0..line.1 {
            let mut head = ropes[0];

            match line.0 {
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                'L' => head.0 -= 1,
                'R' => head.0 += 1,
                _ => panic!("Unexpected direction {}", line.0),
            }

            ropes[0] = head;

            for i in 0..ropes.len() - 1 {
                let (primary, mut secondary) = (ropes[i], ropes[i + 1]);

                if !is_touching(primary, secondary) {
                    let movement_res = move_tail(primary, secondary);
                    secondary = movement_res;
                }

                ropes[i + 1] = secondary;
            }

            part2_visited.insert(*ropes.last().unwrap());
        }
    }

    println!("{}", part2_visited.len());
}
