use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to open example");
    let input_lines: Vec<&str> = input.lines().collect();
    const PREAMBLE_LENGTH: usize = 25;

    let mut preamble: Vec<usize> = Vec::new();
    let mut part_one_answer: usize = 0;

    for i in 0..PREAMBLE_LENGTH {
        preamble.push(input_lines[i].parse::<usize>().unwrap());
    }

    let mut input_usize: Vec<usize> = Vec::new();

    for line in input_lines {
        input_usize.push(line.parse::<usize>().unwrap());
    }

    for i in PREAMBLE_LENGTH..input_usize.len() {
        let mut has_matched = false;

        for j in i - PREAMBLE_LENGTH..i {
            for k in i - PREAMBLE_LENGTH..i {
                if input_usize[j] + input_usize[k] == input_usize[i] {
                    has_matched = true;
                }
            }
        }

        if has_matched == false {
            part_one_answer = input_usize[i];
            println!("Part one answer: {}", input_usize[i]);
        }
    }

    let mut test_range: Vec<usize> = Vec::new();
    for i in 0..input_usize.len() {
        while test_range.iter().sum::<usize>() <= part_one_answer {
            for j in i..input_usize.len() {
                if input_usize[j] != part_one_answer {
                    test_range.push(input_usize[j]);
                }
                if test_range.iter().sum::<usize>() == part_one_answer {
                    test_range.sort();
                    println!(
                        "Part two answer: {}",
                        test_range.first().unwrap() + test_range.last().unwrap()
                    );
                }
            }
        }
        test_range.clear();
    }
}
