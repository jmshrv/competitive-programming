use std::fs;

struct BoardingPass {
    column: usize,
    row: usize,
    seat_id: usize,
}

fn generate_boarding_pass(boarding_pass_string: &str) -> BoardingPass {
    let boarding_pass_chars: Vec<char> = boarding_pass_string.chars().collect();

    let mut rows: Vec<usize> = (0..128).collect();
    let mut columns: Vec<usize> = (0..8).collect();
    for i in 0..7 {
        if boarding_pass_chars[i] == 'B' {
            rows.drain(0..rows.len() / 2);
        } else if boarding_pass_chars[i] == 'F' {
            rows.drain(rows.len() / 2..rows.len());
        } else {
            println!(
                "Invalid character {}! Expected F or B",
                boarding_pass_chars[i]
            );
        }
    }

    if rows.len() != 1 {
        println!(
            "Rows length isn't the expected value! Got {}, expected 1",
            rows.len()
        );
    }

    for i in 7..10 {
        if boarding_pass_chars[i] == 'R' {
            columns.drain(0..columns.len() / 2);
        } else if boarding_pass_chars[i] == 'L' {
            columns.drain(columns.len() / 2..columns.len());
        } else {
            println!(
                "Invalid character {}! Expected L or R",
                boarding_pass_chars[i]
            );
        }
    }

    if columns.len() != 1 {
        println!(
            "Columns length isn't the expected value! Got {}, expected 1",
            columns.len()
        );
    }

    return BoardingPass {
        row: rows[0],
        column: columns[0],
        seat_id: rows[0] * 8 + columns[0],
    };
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let mut processed_boarding_passes: Vec<BoardingPass> = Vec::new();
    for boarding_pass_string in input.lines() {
        processed_boarding_passes.push(generate_boarding_pass(boarding_pass_string));
    }

    let mut seat_ids: Vec<usize> = Vec::new();
    for boarding_pass in &processed_boarding_passes {
        seat_ids.push(boarding_pass.seat_id);
    }

    println!("Part one answer: {}", seat_ids.iter().max().unwrap());

    seat_ids.sort();

    for i in 1..seat_ids.len() - 1 {
        if seat_ids[i] + 1 != seat_ids[i + 1] {
            println!("Part two answer: {}", seat_ids[i] + 1);
        }
    }
}
