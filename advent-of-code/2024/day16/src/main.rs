use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    io,
};

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Start,
    End,
    Empty,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotated_neighbours(&self) -> (Direction, Direction) {
        match self {
            Direction::North => (Direction::East, Direction::West),
            Direction::East => (Direction::South, Direction::North),
            Direction::South => (Direction::West, Direction::East),
            Direction::West => (Direction::North, Direction::South),
        }
    }

    fn vector(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    direction: Direction,
    position: (isize, isize),
    cost: u64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn next_nodes<'a>(&self, map: &'a [Vec<Tile>]) -> impl Iterator<Item = Node> + use<'_, 'a> {
        let forward_pos = (
            self.position.0 + self.direction.vector().0,
            self.position.1 + self.direction.vector().1,
        );

        let (clockwise, anticlockwise) = self.direction.rotated_neighbours();

        let clockwise_pos = (
            self.position.0 + clockwise.vector().0,
            self.position.1 + clockwise.vector().1,
        );

        let anticlockwise_pos = (
            self.position.0 + anticlockwise.vector().0,
            self.position.1 + anticlockwise.vector().1,
        );

        [
            Node {
                direction: self.direction,
                position: forward_pos,
                cost: self.cost + 1,
            },
            Node {
                direction: clockwise,
                position: clockwise_pos,
                cost: self.cost + 1001,
            },
            Node {
                direction: anticlockwise,
                position: anticlockwise_pos,
                cost: self.cost + 1001,
            },
        ]
        .into_iter()
        .filter(|node| {
            map.get(node.position.0 as usize)
                .is_some_and(|row| row.get(node.position.1 as usize).is_some())
        })
        .filter(|node| map[node.position.0 as usize][node.position.1 as usize] != Tile::Wall)
    }
}

fn score(map: &[Vec<Tile>]) -> Option<u64> {
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, tile)| {
                if *tile == Tile::Start {
                    Some((y as isize, x as isize))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let end = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, tile)| {
                if *tile == Tile::End {
                    Some((y as isize, x as isize))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut queue = BinaryHeap::from([Node {
        direction: Direction::East,
        position: start,
        cost: 0,
    }]);

    let mut visited = HashSet::new();

    while let Some(node) = queue.pop() {
        if node.position == end {
            return Some(node.cost);
        }

        if !visited.insert((node.position, node.direction)) {
            continue;
        }

        queue.extend(node.next_nodes(map));
    }

    None
}

fn main() {
    let map = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|row| {
            row.chars()
                .map(|char| match char {
                    'S' => Tile::Start,
                    'E' => Tile::End,
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    _ => panic!("Invalid tile {char}!"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part_one = score(&map).unwrap();

    println!("{part_one}");
}
