use std::io;

fn num_from_line(line: &str, include_str_nums: bool) -> Option<u32> {
    let str_nums = [
        ("one", 1u32),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut matches = vec![];

    if include_str_nums {
        for i in 0..line.len() {
            for str_num in str_nums {
                let slice_to = i + str_num.0.len();

                if slice_to > line.len() {
                    continue;
                }

                let slice = &line[i..slice_to];

                if slice == str_num.0 {
                    matches.push((i, str_num.1))
                }
            }
        }
    }

    let mut nums: Vec<(usize, u32)> = line
        .char_indices()
        .filter_map(|char| {
            let digit = char.1.to_digit(10)?;
            Some((char.0, digit))
        })
        .collect();

    matches.append(&mut nums);

    matches.sort_unstable();

    let first = matches.first()?.1;
    let last = matches.last()?.1;

    let res = first * 10 + last;

    Some(res)
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(|res| res.ok()).collect();

    let part1_result: u32 = lines
        .iter()
        .map(|line| num_from_line(&line, false))
        .map(|opt| opt.unwrap()) // If a line fails I'd like to know
        .sum();

    println!("{part1_result}");

    let part2_result: u32 = lines
        .iter()
        .map(|line| num_from_line(&line, true))
        .map(|opt| opt.unwrap())
        .sum();

    println!("{part2_result}");
}
