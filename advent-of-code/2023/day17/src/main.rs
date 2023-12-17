use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    io,
};

#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
    direction: Direction,
    same_direction_count: usize,
    last: Option<Box<State>>,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Start,
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn valid_next_steps(
        &self,
        same_direction_count: usize,
        min_direction: usize,
        max_direction: usize,
    ) -> Vec<Self> {
        if same_direction_count < min_direction {
            if *self == Direction::Start {
                return vec![Self::Left, Self::Right, Self::Up, Self::Down];
            }
            return vec![*self];
        }

        let mut base_directions = match self {
            Self::Start => vec![Self::Left, Self::Right, Self::Up, Self::Down],
            Self::Left | Self::Right => vec![Self::Up, Self::Down],
            Self::Up | Self::Down => vec![Self::Left, Self::Right],
        };

        if *self != Self::Start && same_direction_count < max_direction {
            base_directions.push(*self);
        }

        base_directions
    }

    fn vector(&self) -> (isize, isize) {
        match self {
            Direction::Start => panic!("Vector called on start!"),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }
}

fn shortest_path(
    input: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    min_direction: usize,
    max_direction: usize,
) -> usize {
    let x_length = input.first().expect("No first line!").len();

    // let mut dist = vec![vec![usize::MAX; x_length]; input.len()];
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost: 0,
        position: start,
        same_direction_count: 0,
        direction: Direction::Start,
        last: None,
    });

    while let Some(state) = heap.pop() {
        let cost = state.cost;
        let (x, y) = state.position;
        let same_direction_count = state.same_direction_count;
        let direction = state.direction;

        // if (x, y) == (5, 1) {
        //     println!("hi!");
        // }

        // Alternatively we could have continued to find all shortest paths
        if (x, y) == end && same_direction_count >= min_direction {
            // let mut last = state.last.unwrap();

            // println!(
            //     "({}, {}) {:?} {}",
            //     last.position.0, last.position.1, last.direction, last.cost
            // );

            // while let Some(ref new_last) = last.last {
            //     last = new_last.clone();

            //     println!(
            //         "({}, {}) {:?} {}",
            //         last.position.0, last.position.1, last.direction, last.cost
            //     );
            // }

            return cost;
        }

        // Important as we may have already found a better way
        // if cost > dist[y][x] {
        //     continue;
        // }

        if !visited.insert((state.position, same_direction_count, direction)) {
            continue;
        }

        let next_steps = direction
            .valid_next_steps(same_direction_count, min_direction, max_direction)
            .into_iter()
            .map(|direction| {
                (
                    (
                        x as isize + direction.vector().0,
                        y as isize + direction.vector().1,
                    ),
                    direction,
                )
            })
            .collect::<Vec<_>>();

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for ((next_x, next_y), next_direction) in next_steps {
            if let Some(weight) = input
                .get(next_y as usize)
                .and_then(|line| line.get(next_x as usize))
            {
                let next = State {
                    cost: cost + (weight.to_digit(10).expect("Failed to convert weight!") as usize),
                    position: (next_x as usize, next_y as usize),
                    direction: next_direction,
                    same_direction_count: if direction == next_direction {
                        same_direction_count + 1
                    } else {
                        1
                    },
                    last: Some(Box::new(state.clone())),
                };

                // if (next_x, next_y) == (5, 0) {
                //     println!("hi!");
                // }

                // If so, add it to the frontier and continue
                // if next.cost < dist[next_y as usize][next_x as usize] {
                heap.push(next.clone());
                // Relaxation, we have now found a better way
                // dist[next_y as usize][next_x as usize] = next.cost;
                // }
            }
        }
    }

    unreachable!()
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let x_length = input.first().expect("No first line!").len();

    let part1_answer = shortest_path(&input, (0, 0), (x_length - 1, input.len() - 1), 0, 3);
    println!("{part1_answer}");

    let part2_answer = shortest_path(&input, (0, 0), (x_length - 1, input.len() - 1), 4, 10);
    println!("{part2_answer}");
}
