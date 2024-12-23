use std::io;

fn mix(secret_number: u64, value: u64) -> u64 {
    value ^ secret_number
}

fn prune(secret_number: u64) -> u64 {
    secret_number % 16777216
}

fn evolve(mut secret_number: u64) -> u64 {
    secret_number = mix(secret_number, secret_number * 64);
    secret_number = prune(secret_number);

    secret_number = mix(secret_number, secret_number / 32);
    secret_number = prune(secret_number);

    secret_number = mix(secret_number, secret_number * 2048);

    prune(secret_number)
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let part_one: u64 = input
        .iter()
        .map(|initial_secret| {
            let mut answer = *initial_secret;

            for _ in 0..2000 {
                answer = evolve(answer);
            }

            answer
        })
        .sum();

    println!("{part_one}");
}
