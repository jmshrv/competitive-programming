use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();

    let ranges = ranges_str
        .lines()
        .map(|range| {
            let (from_str, to_str) = range.split_once('-').unwrap();

            let from = from_str.parse::<u64>().unwrap();
            let to = to_str.parse::<u64>().unwrap();

            from..=to
        })
        .collect::<Vec<_>>();

    let ingredients = ingredients_str
        .lines()
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let part1_answer = ingredients
        .iter()
        .filter(|ingredient| {
            ranges
                .iter()
                .find(|range| range.contains(ingredient))
                .is_some()
        })
        .count();

    println!("{part1_answer}");
}
