use std::io;

/// Parses the input into pairs of times and distances
fn parse_races(input: &[String]) -> Vec<(u64, u64)> {
    let times = input
        .first()
        .expect("Missing first line?")
        .split_ascii_whitespace()
        .skip(1)
        .map(|time_str| time_str.parse::<u64>().expect("Failed to parse time"));

    let distances = input
        .last()
        .expect("Missing last line?")
        .split_ascii_whitespace()
        .skip(1)
        .map(|time_str| time_str.parse::<u64>().expect("Failed to parse distance"));

    times.zip(distances).collect::<Vec<_>>()
}

fn parse_race(input: &[String]) -> (u64, u64) {
    let time = input
        .first()
        .expect("Missing first line?")
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .concat()
        .parse::<u64>()
        .expect("Failed to parse time");

    let distance = input
        .last()
        .expect("Missing last line?")
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .concat()
        .parse::<u64>()
        .expect("Failed to parse distance");

    (time, distance)
}

/// Takes a race, and returns a Vec of holding milliseconds that would beat the record
fn winning_strategies(race: (u64, u64)) -> usize {
    // for i in 0..=race.0 {
    //     let distance_trav
    // }

    (0..=race.0)
        .filter(|hold_time| (hold_time * (race.0 - hold_time)) > race.1)
        .count()
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    let part1_races = parse_races(&input);

    let part1_answer: usize = part1_races
        .iter()
        .map(|race| winning_strategies(*race))
        .product();

    println!("{part1_answer}");

    let part2_race = parse_race(&input);

    let part2_answer = winning_strategies(part2_race);

    println!("{part2_answer}");
}
