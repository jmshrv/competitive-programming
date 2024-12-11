use std::{collections::HashMap, io};

fn blink_count(stone: u64, iter: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if iter == 0 {
        return 1;
    }

    if let Some(result) = cache.get(&(stone, iter)) {
        return *result;
    }

    if stone == 0 {
        let result = blink_count(1, iter - 1, cache);

        cache.insert((stone, iter), result);

        return result;
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

        let result =
            blink_count(*first_half, iter - 1, cache) + blink_count(*second_half, iter - 1, cache);

        cache.insert((stone, iter), result);

        return result;
    }

    let result = blink_count(stone * 2024, iter - 1, cache);

    cache.insert((stone, iter), result);

    result
}

fn blink_all(stones: &[u64], iters: u64) -> u64 {
    let mut cache = HashMap::new();

    stones
        .iter()
        .map(|stone| blink_count(*stone, iters, &mut cache))
        .sum()
}

fn main() {
    let input = io::read_to_string(io::stdin())
        .unwrap()
        .split_whitespace()
        .map(|stone_str| stone_str.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let part_one = blink_all(&input, 25);

    println!("{part_one}");

    let part_two = blink_all(&input, 75);

    println!("{part_two}");
}
