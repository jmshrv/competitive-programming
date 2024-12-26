use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
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

    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn next_nodes<'a>(
        &self,
        map: &'a [Vec<Tile>],
        inverse: bool,
    ) -> impl Iterator<Item = Node> + use<'_, 'a> {
        let forward_pos = if inverse {
            (
                self.position.0 + self.direction.opposite().vector().0,
                self.position.1 + self.direction.opposite().vector().1,
            )
        } else {
            (
                self.position.0 + self.direction.vector().0,
                self.position.1 + self.direction.vector().1,
            )
        };

        let (clockwise, anticlockwise) = self.direction.rotated_neighbours();

        let neighbours = if inverse {
            [
                Node {
                    direction: self.direction,
                    position: forward_pos,
                    cost: self.cost - 1,
                },
                Node {
                    direction: clockwise,
                    position: self.position,
                    cost: self.cost - 1000,
                },
                Node {
                    direction: anticlockwise,
                    position: self.position,
                    cost: self.cost - 1000,
                },
            ]
        } else {
            [
                Node {
                    direction: self.direction,
                    position: forward_pos,
                    cost: self.cost + 1,
                },
                Node {
                    direction: clockwise,
                    position: self.position,
                    cost: self.cost + 1000,
                },
                Node {
                    direction: anticlockwise,
                    position: self.position,
                    cost: self.cost + 1000,
                },
            ]
        };

        neighbours
            .into_iter()
            .filter(|node| {
                map.get(node.position.0 as usize)
                    .is_some_and(|row| row.get(node.position.1 as usize).is_some())
            })
            .filter(|node| map[node.position.0 as usize][node.position.1 as usize] != Tile::Wall)
    }
}

fn score(map: &[Vec<Tile>]) -> Option<(u64, HashMap<((isize, isize), Direction), u64>)> {
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

    let mut visited = HashMap::new();

    while let Some(node) = queue.pop() {
        if node.position == end {
            visited.insert((node.position, node.direction), node.cost);
            return Some((node.cost, visited));
        }

        if visited.contains_key(&(node.position, node.direction)) {
            continue;
        }

        visited.insert((node.position, node.direction), node.cost);

        queue.extend(node.next_nodes(map, false));
    }

    None
}

fn seats(
    visited_one: &HashMap<((isize, isize), Direction), u64>,
    start: &[Node],
) -> HashSet<(isize, isize)> {
    let mut queue = Vec::from(start);

    let mut visited = HashSet::new();

    while let Some(node) = queue.pop() {
        if visited_one.get(&(node.position, node.direction)) == Some(&node.cost)
            && visited.insert((node.position, node.direction))
        {
            let (clockwise, anticlockwise) = node.direction.rotated_neighbours();

            queue.push(Node {
                direction: clockwise,
                position: node.position,
                cost: node.cost - 1000,
            });

            queue.push(Node {
                direction: anticlockwise,
                position: node.position,
                cost: node.cost - 1000,
            });

            queue.push(Node {
                direction: node.direction,
                position: (
                    node.position.0 + node.direction.opposite().vector().0,
                    node.position.1 + node.direction.opposite().vector().1,
                ),
                cost: node.cost - 1,
            });
        }
    }

    visited.into_iter().map(|(position, _)| position).collect()
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

    let (part_one, visited) = score(&map).unwrap();

    println!("{part_one}");

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
        &visited,
        &[
            Node {
                direction: Direction::North,
                position: end,
                cost: part_one,
            },
            Node {
                direction: Direction::East,
                position: end,
                cost: part_one,
            },
            Node {
                direction: Direction::South,
                position: end,
                cost: part_one,
            },
            Node {
                direction: Direction::West,
                position: end,
                cost: part_one,
            },
        ],
    )
    .len();

    println!("{part_two}");
}
