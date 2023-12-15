use std::io;

fn hash(input: &str) -> u64 {
    let mut res = 0;

    for char in input.chars() {
        res += char as u64;
        res *= 17;
        res %= 256;
    }

    res
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .next()
        .expect("No line!");

    let split_input = input.split(',');

    let part1_answer: u64 = split_input.map(|step| hash(step)).sum();

    println!("{part1_answer}");
}
