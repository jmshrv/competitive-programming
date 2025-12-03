use std::io;

fn joltages(bank: &[u32]) -> impl Iterator<Item = u32> {
    (0..bank.len() - 1).flat_map(move |i| {
        (i..bank.len()).filter_map(move |j| {
            if i == j {
                None
            } else {
                Some(bank[i] * 10 + bank[j])
            }
        })
    })
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1_answer: u32 = input.iter().map(|bank| joltages(bank).max().unwrap()).sum();

    println!("{part1_answer}");
}
