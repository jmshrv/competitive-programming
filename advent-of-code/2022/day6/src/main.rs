use std::{collections::HashSet, io};

use itertools::Itertools;

trait AllUnique {
    fn all_unique(self) -> bool;
}

impl AllUnique for (char, char, char, char) {
    fn all_unique(self) -> bool {
        (self.0 != self.1 && self.0 != self.2 && self.0 != self.3)
            && (self.1 != self.2 && self.1 != self.3)
            && (self.2 != self.3)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let answer_1 = input
        .chars()
        .tuple_windows::<(_, _, _, _)>()
        .find_position(|window| window.all_unique())
        .unwrap()
        .0
        + 4;

    let answer_2 = input
        .as_bytes()
        .windows(14)
        .find_position(|window| window.iter().collect::<HashSet<_>>().len() == 14)
        .unwrap()
        .0
        + 14;

    println!("{}", answer_1);
    println!("{}", answer_2);
}
