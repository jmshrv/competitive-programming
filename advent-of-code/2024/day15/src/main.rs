use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Box,
    Empty,
    Robot,
    Wall,
    BoxLeft,
    BoxRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn vector(&self, from_pos: (isize, isize)) -> (isize, isize) {
        match self {
            Direction::Up => (from_pos.0 - 1, from_pos.1),
            Direction::Down => (from_pos.0 + 1, from_pos.1),
            Direction::Left => (from_pos.0, from_pos.1 - 1),
            Direction::Right => (from_pos.0, from_pos.1 + 1),
        }
    }
}

fn parse_map(map_str: &str) -> Vec<Vec<Tile>> {
    map_str
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'O' => Tile::Box,
                    '.' => Tile::Empty,
                    '@' => Tile::Robot,
                    '#' => Tile::Wall,
                    _ => panic!("Invalid character {char}!"),
                })
                .collect()
        })
        .collect()
}

fn parse_directions(directions_str: &str) -> Vec<Direction> {
    directions_str
        .chars()
        .filter_map(|char| match char {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        })
        .collect()
}

fn is_free(map: &[Vec<Tile>], tile_pos: (isize, isize), direction: Direction) -> bool {
    match map[tile_pos.0 as usize][tile_pos.1 as usize] {
        Tile::Empty => true,
        Tile::Wall => false,
        Tile::BoxLeft => {
            is_free(map, direction.vector(tile_pos), direction)
                && is_free(
                    map,
                    direction.vector(Direction::Right.vector(tile_pos)),
                    direction,
                )
        }
        Tile::BoxRight => {
            is_free(map, direction.vector(tile_pos), direction)
                && is_free(
                    map,
                    direction.vector(Direction::Left.vector(tile_pos)),
                    direction,
                )
        }
        _ => is_free(map, direction.vector(tile_pos), direction),
    }
}

fn try_move(
    map: &mut Vec<Vec<Tile>>,
    tile_pos: (isize, isize),
    direction: Direction,
) -> Option<(isize, isize)> {
    let tile = map[tile_pos.0 as usize][tile_pos.1 as usize];

    match tile {
        Tile::Empty => Some(tile_pos),
        Tile::Wall => None,

        Tile::BoxLeft | Tile::BoxRight
            if direction == Direction::Up || direction == Direction::Down =>
        {
            let other_box_pos = if tile == Tile::BoxLeft {
                Direction::Right.vector(tile_pos)
            } else {
                Direction::Left.vector(tile_pos)
            };

            let new_pos = direction.vector(tile_pos);
            let other_new_pos = direction.vector(other_box_pos);

            if !is_free(map, tile_pos, direction) {
                return None;
            }

            if try_move(map, new_pos, direction).is_some()
                && try_move(map, other_new_pos, direction).is_some()
            {
                map[tile_pos.0 as usize][tile_pos.1 as usize] = Tile::Empty;
                map[other_box_pos.0 as usize][other_box_pos.1 as usize] = Tile::Empty;

                map[new_pos.0 as usize][new_pos.1 as usize] = tile;
                map[other_new_pos.0 as usize][other_new_pos.1 as usize] = if tile == Tile::BoxLeft {
                    Tile::BoxRight
                } else {
                    Tile::BoxLeft
                };

                return Some(new_pos);
            }

            None
        }

        // Boxes and robot
        tile => {
            let new_pos = direction.vector(tile_pos);
            if try_move(map, new_pos, direction).is_some() {
                map[tile_pos.0 as usize][tile_pos.1 as usize] = Tile::Empty;

                map[new_pos.0 as usize][new_pos.1 as usize] = tile;

                return Some(new_pos);
            }

            None
        }
    }
}

fn debug_print(map: &[Vec<Tile>]) {
    for row in map {
        for tile in row {
            match tile {
                Tile::Box => print!("O"),
                Tile::Empty => print!("."),
                Tile::Robot => print!("@"),
                Tile::Wall => print!("#"),
                Tile::BoxLeft => print!("["),
                Tile::BoxRight => print!("]"),
            }
        }

        println!();
    }
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (map_str, directions_str) = input.split_once("\n\n").unwrap();

    let mut map = parse_map(map_str);

    let mut extended_map = map
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|cell| match cell {
                    Tile::Box => [Tile::BoxLeft, Tile::BoxRight],
                    Tile::Empty => [Tile::Empty, Tile::Empty],
                    Tile::Robot => [Tile::Robot, Tile::Empty],
                    Tile::Wall => [Tile::Wall, Tile::Wall],
                    _ => panic!("Unexpandable cell {cell:?}!"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let directions = parse_directions(directions_str);

    let mut robot_pos = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|x| *x == Tile::Robot)
                .map(|x| (y as isize, x as isize))
        })
        .unwrap();

    for direction in &directions {
        if let Some(new_pos) = try_move(&mut map, robot_pos, *direction) {
            robot_pos = new_pos;
        }
    }

    let part_one: usize = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, tile)| **tile == Tile::Box)
                .map(move |(x, _)| (y, x))
        })
        .map(|(y, x)| y * 100 + x)
        .sum();

    println!("{part_one}");

    let mut robot_pos = extended_map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|x| *x == Tile::Robot)
                .map(|x| (y as isize, x as isize))
        })
        .unwrap();

    for direction in &directions {
        if let Some(new_pos) = try_move(&mut extended_map, robot_pos, *direction) {
            robot_pos = new_pos;
        }
    }

    let part_two: usize = extended_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, tile)| **tile == Tile::BoxLeft)
                .map(move |(x, _)| (y, x))
        })
        .map(|(y, x)| y * 100 + x)
        .sum();

    println!("{part_two}");
}
