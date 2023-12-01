use std::fs;

fn parse_number(number: &str) -> isize {
    let mut chars: Vec<char> = number.chars().collect();
    if chars[0] == '+' {
        chars.remove(0);
        let number_string: String = chars.into_iter().collect();
        let number = number_string.parse::<isize>().unwrap();
        return 0 + number;
    } else {
        chars.remove(0);
        let number_string: String = chars.into_iter().collect();
        let number = number_string.parse::<isize>().unwrap();
        return 0 - number;
    }
}

fn run_program(input: String, part_two: bool) -> isize {
    let mut accumulator: isize = 0;
    let mut program_counter: isize = 0;
    let mut executed_instruction_numbers: Vec<isize> = Vec::new();
    let mut executed_twice_instruction_numbers: Vec<isize> = Vec::new();
    let mut jmp_switch = 0;
    let mut jmp_counter = 0;
    while program_counter < input.lines().count() as isize {
        if executed_instruction_numbers.contains(&program_counter) {
            if part_two == false {
                break;
            }
            if executed_twice_instruction_numbers.contains(&program_counter) {
                // println!("Looped with {}th jmp switched", jmp_counter);
                jmp_switch += 1;
                jmp_counter = 0;
                program_counter = 0;
                accumulator = 0;
                executed_instruction_numbers.clear();
                executed_twice_instruction_numbers.clear();
            }
            executed_twice_instruction_numbers.push(program_counter);
        }
        executed_instruction_numbers.push(program_counter);
        let lines: Vec<&str> = input.lines().collect();
        let line_split: Vec<&str> = lines[program_counter as usize].split_whitespace().collect();

        match line_split[0] {
            "acc" => {
                accumulator += parse_number(line_split[1]);
                program_counter += 1;
            }
            "jmp" => {
                if part_two == false {
                    program_counter += parse_number(line_split[1]);
                } else if jmp_counter == jmp_switch {
                    program_counter += 1;
                    jmp_counter += 1;
                } else {
                    program_counter += parse_number(line_split[1]);
                    jmp_counter += 1;
                }
            }
            _ => program_counter += 1,
        }
    }
    return accumulator;
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");

    println!("Part one answer: {}", run_program(input.clone(), false));
    println!("Part two answer: {}", run_program(input, true));
}
