use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Box,
    Empty,
    Robot,
    Wall,
}

#[derive(Debug, Clone, Copy)]
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
        .filter(|char| *char != '\n')
        .map(|char| match char {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction {char}!"),
        })
        .collect()
}

fn try_move(
    map: &mut Vec<Vec<Tile>>,
    tile_pos: (isize, isize),
    direction: Direction,
) -> Option<(isize, isize)> {
    // match map.get_mut(&tile_pos) {
    //     Some(tile) => match tile {
    //         Tile::Box => try_move(map, direction.vector(tile_pos), direction),
    //         Tile::Robot => try_move(map, direction.vector(tile_pos), direction),
    //         Tile::Wall => (0, 0), // Walls are immovable
    //     },
    //     None => direction.vector(tile_pos),
    // }

    if tile_pos.0 < 0
        || tile_pos.0 > (map.len() - 1) as isize
        || tile_pos.1 < 0
        || tile_pos.1 > (map[tile_pos.0 as usize].len() - 1) as isize
    {
        return None;
    }

    match map[tile_pos.0 as usize][tile_pos.1 as usize] {
        Tile::Empty => Some(tile_pos),
        Tile::Wall => None,
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
            }
        }

        println!();
    }
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (map_str, directions_str) = input.split_once("\n\n").unwrap();

    let mut map = parse_map(map_str);
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

    for direction in directions {
        if let Some(new_pos) = try_move(&mut map, robot_pos, direction) {
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
}
