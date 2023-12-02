use std::{collections::HashMap, io};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::is_digit,
    combinator::value,
    multi::separated_list1,
    IResult,
};

#[derive(Clone, Eq, Hash, PartialEq)]
enum Colour {
    Red,
    Green,
    Blue,
}

fn parse_line(line: &str) -> IResult<&str, (u32, Vec<HashMap<Colour, u32>>)> {
    // Strip "Game " from start of line
    let (remaining, _) = tag("Game ")(line)?;

    // Get name number and convert into an int
    let (remaining, game_number_str) = take_while1(|c: char| is_digit(c as u8))(remaining)?;
    let game_number = game_number_str
        .parse::<u32>()
        .expect("Failed to parse game number (too lazy to return properly)");

    //  Strip colon
    let (remaining, _) = tag(": ")(remaining)?;

    // let games = remaining.split("; ");

    // for game in games {
    //     let cubes = game.split(", ");

    //     for cube in cubes {
    //         let ()
    //     }
    // }

    let (remaining, hands) =
        separated_list1(tag("; "), take_while1(|c: char| c != ';'))(remaining)?;

    let mut all_hands = vec![];

    for hand in hands {
        let mut cube_map = HashMap::new();

        let (_, cubes) = separated_list1(tag(", "), take_while1(|c: char| c != ','))(hand)?;

        for cube in cubes {
            let (cube_remaining, count_str) = take_while1(|c: char| is_digit(c as u8))(cube)?;
            let cube_count = count_str
                .parse::<u32>()
                .expect("Failed to parse cube count (still too lazy to return properly)");

            let (cube_remaining, _) = tag(" ")(cube_remaining)?;

            let (_, colour) = alt((
                value(Colour::Red, tag("red")),
                value(Colour::Green, tag("green")),
                value(Colour::Blue, tag("blue")),
            ))(cube_remaining)?;

            cube_map.insert(colour, cube_count);
        }

        all_hands.push(cube_map);
    }

    Ok((remaining, (game_number, all_hands)))
}

fn is_game_valid(game: &Vec<HashMap<Colour, u32>>) -> bool {
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    for hand in game {
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        for cube in hand {
            match cube.0 {
                Colour::Red => red_count += cube.1,
                Colour::Green => green_count += cube.1,
                Colour::Blue => blue_count += cube.1,
            }
        }

        if red_count > red_max || green_count > green_max || blue_count > blue_max {
            return false;
        }
    }

    true
}

fn cube_power_set(game: &Vec<HashMap<Colour, u32>>) -> u32 {
    let mut highest_red = 0;
    let mut highest_green = 0;
    let mut highest_blue = 0;

    for hand in game {
        for cube in hand {
            match cube.0 {
                Colour::Red => {
                    if *cube.1 > highest_red {
                        highest_red = *cube.1
                    }
                }
                Colour::Green => {
                    if *cube.1 > highest_green {
                        highest_green = *cube.1
                    }
                }
                Colour::Blue => {
                    if *cube.1 > highest_blue {
                        highest_blue = *cube.1
                    }
                }
            }
        }
    }

    highest_red * highest_green * highest_blue
}

fn main() {
    let input: Vec<String> = io::stdin().lines().filter_map(|res| res.ok()).collect();

    let parsed_games = input
        .iter()
        .map(|line| parse_line(&line))
        .map(|res| res.expect("Failed to parse line"))
        .collect::<Vec<_>>();

    let part1_result: u32 = parsed_games
        .iter()
        .map(|nom_res| &nom_res.1)
        .filter(|game| is_game_valid(&game.1))
        .map(|valid_game| valid_game.0)
        .sum();

    println!("{part1_result}");

    let part2_result: u32 = parsed_games
        .iter()
        .map(|nom_res| &nom_res.1)
        .map(|game| cube_power_set(&game.1))
        .sum();

    println!("{part2_result}");
}
