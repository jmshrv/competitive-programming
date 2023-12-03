use std::{collections::VecDeque, io};

#[derive(Debug)]
struct PartNumber {
    value: u32,
    from: usize,
    to: usize,
}

impl PartNumber {
    pub fn new(value: u32, from: usize, to: usize) -> Self {
        Self { value, from, to }
    }

    pub fn is_gear_aligned(&self, gear_j: usize, line_size: usize) -> bool {
        if gear_j >= self.from && gear_j <= self.to {
            return true;
        }

        let min_gear_diff = (gear_j as isize - self.min_gear_index() as isize).abs();
        let max_gear_diff = (gear_j as isize - self.max_gear_index(line_size) as isize).abs();

        min_gear_diff == 0 || max_gear_diff == 0
    }

    fn min_gear_index(&self) -> usize {
        if self.from == 0 {
            return 0;
        }

        self.from - 1
    }

    fn max_gear_index(&self, line_size: usize) -> usize {
        if self.to == line_size {
            return self.to;
        }

        self.to + 1
    }
}

fn get_part_numbers(input: &Vec<String>) -> Vec<Vec<PartNumber>> {
    let mut part_numbers = vec![];

    for line in input {
        let mut line_part_numbers = vec![];

        let mut current_capture = VecDeque::new();

        for char in line.char_indices() {
            if char.1.is_ascii_digit() {
                current_capture.push_back(char)
            } else {
                if !current_capture.is_empty() {
                    let part_number = part_number_from_dequeue(&mut current_capture);
                    line_part_numbers.push(part_number)
                }
            }
        }

        if !current_capture.is_empty() {
            let part_number = part_number_from_dequeue(&mut current_capture);
            line_part_numbers.push(part_number)
        }

        part_numbers.push(line_part_numbers)
    }

    part_numbers
}

fn part_number_from_dequeue(current_capture: &mut VecDeque<(usize, char)>) -> PartNumber {
    let mut value = 0;
    let from = current_capture.front().unwrap().0;
    let to = current_capture.back().unwrap().0;

    while let Some(captured_char) = current_capture.pop_front() {
        let captured_char_int = captured_char.1.to_digit(10).expect("Invalid number char?");

        value += captured_char_int * 10_u32.pow(current_capture.len() as u32);
    }

    let part_number = PartNumber::new(value, from, to);
    part_number
}

fn is_adj(input: &Vec<String>, line: usize, part_number: &PartNumber) -> bool {
    let i_start = if line == 0 { 0 } else { line - 1 };
    let i_end = if line == input.len() - 1 {
        line
    } else {
        line + 1
    };

    for i in i_start..=i_end {
        let line = &input[i];

        let j_start = if part_number.from == 0 {
            0
        } else {
            part_number.from - 1
        };

        let j_end = if part_number.to == line.chars().count() - 1 {
            part_number.to
        } else {
            part_number.to + 1
        };

        for j in j_start..=j_end {
            let char = line.chars().nth(j).expect("Failed to get char in is_adj");

            if char != '.' && !char.is_ascii_digit() {
                return true;
            }
        }
    }

    false
}

fn get_gears(input: &Vec<String>) -> Vec<(usize, usize)> {
    let mut gears = vec![];

    for i in 0..input.len() {
        let line = &input[i];

        for j in 0..line.chars().count() {
            let char = line.chars().nth(j).expect("Failed to get gear char");

            if char == '*' {
                gears.push((i, j));
            }
        }
    }

    gears
}

fn main() {
    let input = io::stdin()
        .lines()
        .filter_map(|res| res.ok())
        .collect::<Vec<_>>();

    let part_numbers = get_part_numbers(&input);

    // let part1_answer: u32 = part_numbers
    //     .iter()
    //     .enumerate()
    //     .map(|part_number_line| {
    //         part_number_line
    //             .1
    //             .iter()
    //             .filter(|part_number| !is_adj(&input, part_number_line.0, part_number))
    //     })
    //     .map(|valid_part_number_line| {
    //         valid_part_number_line
    //             .map(|valid_part_number| valid_part_number.value)
    //             .sum::<u32>()
    //     })
    //     .sum();

    let mut part1_answer = 0;

    for part_number_line in part_numbers.iter().enumerate() {
        for part_number in part_number_line.1 {
            if is_adj(&input, part_number_line.0, &part_number) {
                part1_answer += part_number.value
            }
        }
    }

    println!("{part1_answer}");

    let gears = get_gears(&input);

    let line_size = input.first().unwrap().chars().count();

    let mut part2_answer = 0;

    for gear in gears {
        let (gear_i, gear_j) = gear;

        let i_start = if gear_i == 0 { 0 } else { gear_i - 1 };
        let i_end = if gear_i == part_numbers.len() - 1 {
            gear_i
        } else {
            gear_i + 1
        };

        let mut valid_part_numbers = vec![];

        for i in i_start..=i_end {
            let part_number_lines = &part_numbers[i];

            let mut valid_part_numbers_line = part_number_lines
                .iter()
                // this shouldn't be absolute
                // .filter(|part_number| {
                //     part_number.from.abs_diff(gear_j) <= 1
                //         && part_number.to.abs_diff(gear_j) > gear_j
                // })
                .filter(|part_number| part_number.is_gear_aligned(gear_j, line_size))
                .collect::<Vec<_>>();

            valid_part_numbers.append(&mut valid_part_numbers_line)
        }

        if valid_part_numbers.len() == 2 {
            part2_answer += valid_part_numbers[0].value * valid_part_numbers[1].value;
            continue;
        }
    }

    println!("{part2_answer}");
}
