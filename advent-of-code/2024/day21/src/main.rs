use std::{
    collections::{HashSet, VecDeque},
    io,
};

trait Keypad<const COLS: usize, const ROWS: usize> {
    const LAYOUT: [[Option<char>; COLS]; ROWS];

    fn neighbours(position: (usize, usize)) -> impl Iterator<Item = ((usize, usize), char)> {
        [
            (position.0 - 1, position.1),
            (position.0 + 1, position.1),
            (position.0, position.1 - 1),
            (position.0, position.1 + 1),
        ]
        .into_iter()
        .filter_map(
            |neighbour| match Self::LAYOUT.get(neighbour.0)?.get(neighbour.1) {
                Some(char_opt) => char_opt.and_then(|char| Some((neighbour, char))),
                None => None,
            },
        )
    }

    fn shortest_path(from: (usize, usize), key: char) -> Vec<(usize, usize)> {
        let mut queue = VecDeque::from([(from, Vec::new())]);
        let mut visited = HashSet::new();

        while let Some((position, path)) = queue.pop_front() {
            if !visited.insert(position) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(position);

            if Self::LAYOUT[position.0][position.1].unwrap() == key {
                return new_path;
            }

            let neighbours =
                Self::neighbours(position).map(|neighbour| (neighbour.0, new_path.clone()));

            queue.extend(neighbours);
        }

        panic!("Unable to find path!")
    }
}

struct NumericKeypad {}

impl Keypad<3, 4> for NumericKeypad {
    #[rustfmt::skip]
    const LAYOUT: [[Option<char>; 3]; 4] = [
        [Some('7'), Some('8'), Some('9')],
        [Some('4'), Some('5'), Some('6')],
        [Some('1'), Some('2'), Some('3')],
        [None,      Some('0'), Some('A')],
    ];
}

fn main() {
    let input = io::stdin().lines().map(Result::unwrap).collect::<Vec<_>>();
}
