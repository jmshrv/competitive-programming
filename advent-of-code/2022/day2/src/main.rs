use std::io;

use itertools::Itertools;

#[derive(Clone, Copy)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scisors = 3,
}

impl TryFrom<char> for Play {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scisors),
            _ => Err(()),
        }
    }
}

impl From<(Play, Outcome)> for Play {
    fn from(play: (Play, Outcome)) -> Self {
        let (opponent, outcome) = play;
        match outcome {
            Outcome::Win => match opponent {
                Play::Rock => Self::Paper,
                Play::Paper => Self::Scisors,
                Play::Scisors => Self::Rock,
            },
            Outcome::Draw => opponent,
            Outcome::Loss => match opponent {
                Play::Rock => Self::Scisors,
                Play::Paper => Self::Rock,
                Play::Scisors => Self::Paper,
            },
        }
    }
}

impl Play {
    fn score(self, play: Play) -> usize {
        let outcome = Outcome::from((self, play));

        outcome as usize + play as usize
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl From<(Play, Play)> for Outcome {
    fn from(play: (Play, Play)) -> Self {
        match play {
            (Play::Rock, Play::Paper) => Self::Win,
            (Play::Rock, Play::Scisors) => Self::Loss,
            (Play::Paper, Play::Rock) => Self::Loss,
            (Play::Paper, Play::Scisors) => Self::Win,
            (Play::Scisors, Play::Rock) => Self::Win,
            (Play::Scisors, Play::Paper) => Self::Loss,
            _ => Self::Draw,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

fn main() {
    let lines = io::stdin().lines().map(|x| x.unwrap()).collect_vec();
    let input_one: Vec<(Play, Play)> = lines
        .iter()
        .map(|x| {
            x.split(' ')
                .take(2)
                .map(|y| Play::try_from(y.chars().next().unwrap()).unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let score_part_1: usize = input_one.iter().map(|x| x.0.score(x.1)).sum();

    let input_two: Vec<(Play, Outcome)> = lines
        .iter()
        .map(|x| {
            x.split(' ')
                .take(2)
                .collect_tuple::<(&str, &str)>()
                .map(|y| {
                    (
                        Play::try_from(y.0.chars().next().unwrap()).unwrap(),
                        Outcome::try_from(y.1.chars().next().unwrap()).unwrap(),
                    )
                })
                .unwrap()
        })
        .collect();

    let score_part_2: usize = input_two.iter().map(|x| x.0.score(Play::from(*x))).sum();

    println!("{}", score_part_1);
    println!("{}", score_part_2);
}
