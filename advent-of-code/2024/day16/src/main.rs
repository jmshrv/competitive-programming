use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    io, u64,
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    direction: Direction,
    position: (isize, isize),
    cost: u64,
    path: HashSet<((u8, u8), Direction)>,
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

        let mut forward_path = self.path.clone();
        let mut clockwise_path = self.path.clone();
        let mut anticlockwise_path = self.path.clone();

        let forward_fresh =
            forward_path.insert(((forward_pos.0 as u8, forward_pos.1 as u8), self.direction));
        let clockwise_fresh =
            clockwise_path.insert(((clockwise_pos.0 as u8, clockwise_pos.1 as u8), clockwise));
        let anticlockwise_fresh = anticlockwise_path.insert((
            (anticlockwise_pos.0 as u8, anticlockwise_pos.1 as u8),
            anticlockwise,
        ));

        let mut nodes = Vec::new();

        if forward_fresh {
            nodes.push(Node {
                direction: self.direction,
                position: forward_pos,
                cost: self.cost + 1,
                path: forward_path,
            });
        }

        if clockwise_fresh {
            nodes.push(Node {
                direction: clockwise,
                position: clockwise_pos,
                cost: self.cost + 1001,
                path: clockwise_path,
            });
        }

        if anticlockwise_fresh {
            nodes.push(Node {
                direction: anticlockwise,
                position: anticlockwise_pos,
                cost: self.cost + 1001,
                path: anticlockwise_path,
            });
        }
        nodes
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
        path: HashSet::from([((start.0 as u8, start.1 as u8), Direction::East)]),
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

fn seats(
    map: &[Vec<Tile>],
    max_cost: u64,
    start: Node,
    end: (isize, isize),
) -> HashSet<((u8, u8), Direction)> {
    let mut travelled = HashSet::new();

    for neighbour in start
        .next_nodes(map)
        .filter(|neighbour| neighbour.cost <= max_cost)
    {
        if neighbour.position == end {
            return neighbour.path;
        } else {
            travelled.extend(seats(map, max_cost, neighbour, end));
        }
    }

    travelled
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

    let part_two = seats(
        &map,
        part_one,
        Node {
            direction: Direction::East,
            position: start,
            cost: 0,
            path: HashSet::from([((start.0 as u8, start.1 as u8), Direction::East)]),
        },
        end,
    )
    .iter()
    .map(|(pos, _)| pos)
    .collect::<HashSet<_>>()
    .len();

    println!("{part_two}");
}
