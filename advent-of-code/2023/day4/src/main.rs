use std::{
    collections::{HashMap, HashSet},
    io,
};

use nom::{
    bytes::complete::{tag, take_until1, take_while1},
    IResult,
};

#[derive(Debug)]
struct ScratchCard {
    card_number: u32,
    winning_numbers: HashSet<u32>,
    play_numbers: HashSet<u32>,
}

impl ScratchCard {
    fn new(card_number: u32, winning_numbers: HashSet<u32>, play_numbers: HashSet<u32>) -> Self {
        Self {
            card_number,
            winning_numbers,
            play_numbers,
        }
    }

    fn matches(&self) -> HashSet<&u32> {
        self.play_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .collect::<HashSet<_>>()
    }
}

fn parse_line(line: &str) -> IResult<&str, ScratchCard> {
    let (remaining, _) = tag("Card")(line)?;
    let (remaining, _) = take_while1(|c: char| c.is_ascii_whitespace())(remaining)?;

    let (remaining, card_number_str) = take_while1(|c: char| c.is_ascii_digit())(remaining)?;
    let card_number = card_number_str
        .parse::<u32>()
        .expect("Failed to parse card number");

    let (remaining, _) = tag(":")(remaining)?;

    let (remaining, winning_number_strings) = take_until1("|")(remaining)?;

    let winning_numbers = winning_number_strings
        .split_ascii_whitespace()
        .map(|number_string| {
            number_string
                .parse::<u32>()
                .expect(&format!("Invalid integer {number_string}"))
        })
        .collect::<HashSet<_>>();

    let (remaining, _) = tag("|")(remaining)?;

    let play_numbers = remaining
        .split_ascii_whitespace()
        .map(|number_string| number_string.parse::<u32>().expect("Invalid integer"))
        .collect::<HashSet<_>>();

    let scratch_card = ScratchCard::new(card_number, winning_numbers, play_numbers);

    // A bit cheeky, but we should have parsed the whole line now
    Ok(("", scratch_card))
}

fn score(matches: usize) -> u32 {
    let mut score = 0;

    for _ in 0..matches {
        if score == 0 {
            score = 1;
        } else {
            score *= 2;
        }
    }

    score
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    let scratch_cards_nom = input
        .iter()
        .map(|line| parse_line(&line))
        .map(|res| res.expect("Failed to parse line"))
        .collect::<Vec<_>>();

    let scratch_cards = scratch_cards_nom
        .iter()
        .map(|nom_res| &nom_res.1)
        .collect::<Vec<_>>();

    let part1_answer: u32 = scratch_cards
        .iter()
        .map(|scratch_card| scratch_card.matches())
        .map(|matches| score(matches.len()))
        .sum();

    // let scratch_cards_copy = scratch_cards
    //     .iter()
    //     .map(|card| (card.card_number, vec![card]))
    //     .collect::<HashMap<_, _>>();

    // for i in 1..scratch_cards.len() as u32 + 1 {
    //     let card = scratch_cards_copy.get(&i).expect("Failed to find card");

    //     for copy in card {

    //     }
    // }

    println!("{part1_answer}");

    let mut card_counts = (1..scratch_cards.len() + 1)
        .map(|card_num| (card_num, 1))
        .collect::<HashMap<_, _>>();

    for i in 0..scratch_cards.len() {
        let card = scratch_cards[i];

        let card_count = card_counts
            .get(&(i + 1))
            .expect("Failed to find card count");

        for _ in 0..*card_count {
            for j in 0..card.matches().len() {
                let next_card_index = i + j + 2;

                // println!(
                //     "Adding card {} from card {}",
                //     next_card_index, card.card_number
                // );

                card_counts
                    .entry(next_card_index)
                    .and_modify(|next_card| *next_card += 1);
            }
        }
    }

    // println!("{:?}", card_counts);
    let part2_answer: u32 = card_counts.values().sum();

    println!("{part2_answer}");
}
