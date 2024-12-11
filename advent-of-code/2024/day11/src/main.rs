use std::io;

fn blink(stones: &[u64]) -> impl Iterator<Item = u64> + use<'_> {
    stones.iter().flat_map(|stone| {
        if *stone == 0 {
            return vec![1];
        }
        let stone_digits = stone.ilog10() + 1;
        if stone_digits % 2 == 0 {
            let stone_str = stone.to_string();

            let first_half = &stone_str[..stone_digits as usize / 2]
                .parse::<u64>()
                .unwrap();
            let second_half = &stone_str[stone_digits as usize / 2..]
                .parse::<u64>()
                .unwrap();

            return vec![*first_half, *second_half];
        }

        vec![*stone * 2024]
    })
}

fn main() {
    let input = io::read_to_string(io::stdin())
        .unwrap()
        .split_whitespace()
        .map(|stone_str| stone_str.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut part_one_stones = input.clone();

    for _ in 0..25 {
        part_one_stones = blink(&part_one_stones).collect();
    }

    let part_one = part_one_stones.len();

    println!("{part_one}");
}
