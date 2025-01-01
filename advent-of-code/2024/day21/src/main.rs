use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    io,
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn key(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }

    fn vectored(&self, from: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (from.0 - 1, from.1),
            Direction::Down => (from.0 + 1, from.1),
            Direction::Left => (from.0, from.1 - 1),
            Direction::Right => (from.0, from.1 + 1),
        }
    }
}

trait Keypad<const COLS: usize, const ROWS: usize> {
    const LAYOUT: [[Option<char>; COLS]; ROWS];

    fn get_parent(&self) -> Option<&DirectionalKeypad>;

    fn get_parent_mut(&mut self) -> Option<&mut DirectionalKeypad>;

    fn get_position(&self) -> (usize, usize);

    fn set_position(&mut self, new_position: (usize, usize));

    fn key_pos(key: Option<char>) -> Option<(usize, usize)> {
        Self::LAYOUT.iter().enumerate().find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, row_key)| (*row_key == key).then_some((y, x)))
        })
    }

    fn neighbours(
        position: (usize, usize),
    ) -> impl Iterator<Item = (Direction, (usize, usize), char)> {
        Direction::iter()
            .map(move |direction| (direction, direction.vectored(position)))
            .filter_map(|(direction, neighbour)| {
                match Self::LAYOUT.get(neighbour.0)?.get(neighbour.1) {
                    Some(char_opt) => char_opt.and_then(|char| Some((direction, neighbour, char))),
                    None => None,
                }
            })
    }

    /// Presses the given button, returning the cost of pressing that button.
    fn press(&mut self, key: char) -> u64 {
        let cost = self.shortest_path(self.get_position(), key).0;

        self.set_position(Self::key_pos(Some(key)).unwrap());

        cost
    }

    fn shortest_path(&self, from: (usize, usize), key: char) -> (u64, Vec<(usize, usize)>) {
        let mut queue =
            BinaryHeap::from([Reverse((0, from, Vec::new(), self.get_parent().cloned()))]);
        let mut visited = HashSet::new();

        while let Some(Reverse((cost, position, path, parent))) = queue.pop() {
            if !visited.insert(position) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(position);

            if Self::LAYOUT[position.0][position.1].unwrap() == key {
                return (cost, new_path);
            }

            let neighbours = Self::neighbours(position).map(|(direction, neighbour_pos, _)| {
                Reverse((
                    cost + parent
                        .as_ref()
                        .and_then(|parent| {
                            Some(parent.shortest_path(parent.position, direction.key()).0)
                        })
                        .unwrap_or(1),
                    neighbour_pos,
                    new_path.clone(),
                    self.get_parent().and_then(|parent| {
                        Some(parent.with_position(
                            DirectionalKeypad::key_pos(Some(direction.key())).unwrap(),
                        ))
                    }),
                ))
            });

            queue.extend(neighbours);
        }

        panic!("Unable to find path!")
    }

    fn with_position(&self, position: (usize, usize)) -> Self
    where
        Self: Clone,
    {
        let mut temp = self.clone();
        temp.set_position(position);

        temp
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct NumericKeypad {
    position: (usize, usize),
    parent: DirectionalKeypad,
}

impl Keypad<3, 4> for NumericKeypad {
    #[rustfmt::skip]
    const LAYOUT: [[Option<char>; 3]; 4] = [
        [Some('7'), Some('8'), Some('9')],
        [Some('4'), Some('5'), Some('6')],
        [Some('1'), Some('2'), Some('3')],
        [None,      Some('0'), Some('A')],
    ];

    fn get_parent(&self) -> Option<&DirectionalKeypad> {
        Some(&self.parent)
    }

    fn get_parent_mut(&mut self) -> Option<&mut DirectionalKeypad> {
        Some(&mut self.parent)
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn set_position(&mut self, new_position: (usize, usize)) {
        self.position = new_position;
    }
}

impl NumericKeypad {
    fn new(parent: DirectionalKeypad) -> Self {
        Self {
            position: (3, 2),
            parent,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct DirectionalKeypad {
    position: (usize, usize),
    parent: Option<Box<DirectionalKeypad>>,
}

impl Keypad<3, 2> for DirectionalKeypad {
    #[rustfmt::skip]
    const LAYOUT: [[Option<char>; 3]; 2] = [
        [None,      Some('^'), Some('A')],
        [Some('<'), Some('v'), Some('>')]
    ];

    fn get_parent(&self) -> Option<&DirectionalKeypad> {
        self.parent.as_deref()
    }

    fn get_parent_mut(&mut self) -> Option<&mut DirectionalKeypad> {
        self.parent.as_deref_mut()
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn set_position(&mut self, new_position: (usize, usize)) {
        self.position = new_position;
    }
}

impl DirectionalKeypad {
    fn new(parent: Option<DirectionalKeypad>) -> Self {
        Self {
            position: (0, 2),
            parent: parent.and_then(|parent| Some(Box::new(parent))),
        }
    }
}

fn main() {
    let input = io::stdin().lines().map(Result::unwrap).collect::<Vec<_>>();

    let part_one: u64 = input
        .iter()
        .map(|code| {
            let mut keypad_chain = NumericKeypad::new(DirectionalKeypad::new(Some(
                DirectionalKeypad::new(Some(DirectionalKeypad::new(None))),
            )));

            let numeric_part = code[..code.len() - 1].parse::<u64>().unwrap();

            code.chars()
                .map(|char| keypad_chain.press(char))
                .sum::<u64>()
                + numeric_part
        })
        .sum();

    println!("{part_one}");
}
