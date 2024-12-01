use std::io;

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    let (potionCountStr, timeStr) = input.first().unwrap().split_once(' ').unwrap();

    let potionCount = potionCountStr.parse::<usize>().unwrap();
    let time = timeStr.parse::<usize>().unwrap();
}
