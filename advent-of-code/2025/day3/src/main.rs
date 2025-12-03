use std::io;

fn max_joltage(bank: &[u64], battery_count: usize) -> u64 {
    let mut result = 0;

    // Index that we last picked a battery from
    let mut last_index = 0;

    // The best battery is the highest (and earliest) digit that still leaves room to pick the
    // remaining batteries.
    for remaining in (0..battery_count).rev() {
        let (new_index, best_battery) = bank[last_index..bank.len() - remaining]
            .iter()
            .enumerate()
            .rev() // Needed so that max always picks the first battery
            .max_by_key(|(_, battery)| *battery)
            .unwrap();

        last_index += new_index + 1; // Do +1 to start the next search after this battery

        result = result * 10 + best_battery;
    }

    result
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1_answer: u64 = input.iter().map(|bank| max_joltage(bank, 2)).sum();

    println!("{part1_answer}");

    let part2_answer: u64 = input.iter().map(|bank| max_joltage(bank, 12)).sum();

    println!("{part2_answer}");
}
