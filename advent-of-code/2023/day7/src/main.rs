use std::{cmp::Ordering, io};

use itertools::Itertools;

// I'd rather use a tuple, but I want to do list ops on hands
type Hand = [Card; 5];

trait HandTrait {
    fn card_match(&self, joker_rule: bool) -> Match;

    fn ordering(&self, other: &Self, joker_rule: bool) -> Ordering;

    fn from_str(s: &str) -> Result<Hand, &str>;
}

impl HandTrait for Hand {
    fn card_match(&self, joker_rule: bool) -> Match {
        if joker_rule {
            let mut joker_hands = vec![];

            let joker_indices = self
                .iter()
                .enumerate()
                .filter(|card| *card.1 == Card::Jack)
                .map(|card| card.0)
                .collect::<Vec<_>>();

            if joker_indices.is_empty() {
                return self.card_match(false);
            }

            for index in joker_indices {
                for card in Card::CARDS {
                    let mut new_hand = self.clone();
                    new_hand[index] = Card::Joker(Box::new(card));
                    joker_hands.push(new_hand);
                }
            }

            return joker_hands
                .iter()
                .map(|hand| hand.card_match(true))
                .max()
                .expect("No joker cards?");
        }

        let card_counts = self
            .iter()
            .map(|card| {
                if let Card::Joker(true_card) = card {
                    true_card
                } else {
                    card
                }
            })
            .counts();

        // In a 5 of a kind, the unique cards set will have 1 element
        if card_counts.len() == 1 {
            return Match::FiveOfAKind;
        }

        // A 4 of a kind will have a card of count 4 (amazing I know)
        if let Some(_) = card_counts.iter().find(|entry| *entry.1 == 4) {
            return Match::FourOfAKind;
        }

        if let Some(_) = card_counts.iter().find(|entry| *entry.1 == 3) {
            // If we have a card that occurs 3 times and a card that occurs 2 times, we have a full
            // house. Otherwise, we have a three of a kind.
            if card_counts.values().contains(&2) {
                return Match::FullHouse;
            }

            return Match::ThreeOfAKind;
        }

        if let Some(two_card) = card_counts.iter().find(|entry| *entry.1 == 2) {
            if let Some(_) = card_counts
                .iter()
                .find(|entry| *entry.1 == 2 && entry.0 != two_card.0)
            {
                return Match::TwoPair;
            }

            return Match::OnePair;
        }

        Match::HighCard
    }

    fn ordering(&self, other: &Self, joker_rule: bool) -> Ordering {
        let self_match = self.card_match(joker_rule);
        let other_match = other.card_match(joker_rule);

        match self_match.cmp(&other_match) {
            std::cmp::Ordering::Less => Ordering::Less,
            std::cmp::Ordering::Greater => Ordering::Greater,
            std::cmp::Ordering::Equal => {
                let true_self = if joker_rule {
                    self.clone().map(|card| {
                        if card == Card::Jack {
                            Card::Joker(Box::new(Card::Two))
                        } else {
                            card
                        }
                    })
                } else {
                    self.clone().map(|card| card)
                };

                let true_other = if joker_rule {
                    other.clone().map(|card| {
                        if card == Card::Jack {
                            Card::Joker(Box::new(Card::Two))
                        } else {
                            card
                        }
                    })
                } else {
                    other.clone().map(|card| card)
                };

                for i in 0..true_self.len() {
                    let self_val = &true_self[i];
                    let other_val = &true_other[i];

                    let val_cmp = self_val.cmp(&other_val);

                    if val_cmp != Ordering::Equal {
                        return val_cmp;
                    }
                }

                return Ordering::Equal;
            }
        }
    }

    fn from_str(s: &str) -> Result<Self, &str> {
        s.chars()
            .map(|c| c.try_into().expect("Failed to parse card"))
            .collect::<Vec<Card>>()
            .try_into()
            .or(Err(s))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialOrd, Ord, PartialEq)]
enum Card {
    Joker(Box<Card>),
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    const CARDS: [Card; 12] = [
        Card::Two,
        Card::Three,
        Card::Four,
        Card::Five,
        Card::Six,
        Card::Seven,
        Card::Eight,
        Card::Nine,
        Card::T,
        // Card::Jack,
        Card::Queen,
        Card::King,
        Card::Ace,
    ];

    // fn true_cmp(&self, other: &Self) -> Ordering {
    //     match (self, other) {
    //         (Card::Joker(_), Card::Joker(_)) => Ordering::Equal,
    //         (Card::Joker(_), _) => Ordering::Less,
    //         (_, Card::Joker(_)) => Ordering::Greater,
    //         _ => self.cmp(other),
    //     }
    // }
}

impl TryFrom<char> for Card {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::T),
            // _ => {
            //     if let Some(number) = value.to_digit(10) {
            //         if number >= 2 {
            //             Ok(Self::Number(number as u8)) // Will never fail - to_digit never returns > 9
            //         } else {
            //             Err(value)
            //         }
            //     } else {
            //         Err(value)
            //     }
            // }
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Match {
    // HighCard(Card),
    // OnePair(Card),
    // TwoPair(Card, Card),
    // ThreeOfAKind(Card),
    // FullHouse(Hand),
    // FourOfAKind(Card),
    // FiveOfAKind(Hand),
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse_line(line: &str) -> Result<(Hand, u64), &str> {
    let (hand_str, bid_str) = line.split_once(" ").ok_or(line)?;

    let hand = Hand::from_str(hand_str)?;
    let bid = bid_str.parse::<u64>().or(Err(line))?;

    Ok((hand, bid))
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .map(|line| parse_line(&line).expect("Failed to parse line"))
        .collect::<Vec<_>>();

    let part1_answer: usize = input
        .iter()
        .sorted_by(|a, b| a.0.ordering(&b.0, false))
        .enumerate()
        .map(|(index, (_, bid))| (index + 1) * *bid as usize)
        .sum();

    println!("{part1_answer}");

    let part2_answer: usize = input
        .iter()
        .sorted_by(|a, b| a.0.ordering(&b.0, true))
        // .map(|hand| {
        //     println!("{:?} {:?}", hand.0, hand.0.card_match(true));
        //     hand
        // })
        .enumerate()
        .map(|(index, (_, bid))| (index + 1) * *bid as usize)
        .sum();

    println!("{part2_answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joker_matches() {
        // JKKK2
        let hand1 = [Card::Jack, Card::King, Card::King, Card::King, Card::Two];

        // QQQQ2
        let hand2 = [
            Card::Queen,
            Card::Queen,
            Card::Queen,
            Card::Queen,
            Card::Two,
        ];

        assert_eq!(hand1.ordering(&hand2, true), Ordering::Less);
        assert!(Card::Joker(Box::new(Card::Ace)) < Card::Two)
    }

    //     #[test]
    //     fn matches_five_of_a_kind() {
    //         let hand = [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace];
    //         let res = hand.card_match();

    //         assert_eq!(res, Match::FiveOfAKind(hand));
    //     }

    //     #[test]
    //     fn matches_four_of_a_kind() {
    //         let hand = [Card::Ace, Card::Ace, Card::Eight, Card::Ace, Card::Ace];
    //         let res = hand.card_match();

    //         assert_eq!(res, Match::FourOfAKind(Card::Ace));
    //     }

    //     #[test]
    //     fn matches_full_house() {
    //         let hand = [Card::Two, Card::Three, Card::Three, Card::Three, Card::Two];
    //         let res = hand.card_match();

    //         assert_eq!(res, Match::FullHouse(hand));
    //     }

    //     #[test]
    //     fn matches_three_of_a_kind() {
    //         let hand = [Card::T, Card::T, Card::T, Card::Nine, Card::Eight];
    //         let res = hand.card_match();

    //         assert_eq!(res, Match::ThreeOfAKind(Card::T));
    //     }

    //     #[test]
    //     fn matches_two_pair() {
    //         let hand = [Card::Two, Card::Three, Card::Four, Card::Three, Card::Two];
    //         let res = hand.card_match();

    //         assert_eq!(res, Match::TwoPair(Card::Two, Card::Three));
    //     }

    //     #[test]
    //     fn matches_one_pair() {
    //         let hand = [Card::Ace, Card::Two, Card::Three, Card::Ace, Card::Four];
    //         let res = hand.card_match();

    //         assert_eq!(res, Match::OnePair(Card::Ace));
    //     }

    //     #[test]
    //     fn matches_high_card() {
    //         let hand = [Card::Two, Card::Three, Card::Four, Card::Five, Card::Six];
    //         let res = hand.card_match();

    //         assert_eq!(res, Match::HighCard(Card::Six));
    //     }
}
