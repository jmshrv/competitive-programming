use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");

    let mut group_answers: Vec<Vec<char>> = Vec::new();
    let mut group_answers_temp: Vec<char> = Vec::new();

    for line in input.lines() {
        if line.chars().count() == 0 {
            // Appends the temp value to the full vector.
            // We do this when the count is zero because an empty line is used to separate values
            //
            //
            //
            //
            //
            //
            //
            //
            //
            //
            // WARNING!
            // Using this method means that the last line is never counted, meaning that the part one answer can be off by one.
            //
            //
            //
            //
            //
            //
            //
            //
            //
            //
            group_answers.push(group_answers_temp.clone());
            group_answers_temp.clear();
        } else {
            // Splits the current line by whitespace and appends it to the passport_temp variable
            for character in line.chars() {
                group_answers_temp.push(character);
            }
        }
    }

    let mut answer_count = 0;
    for group_answer in group_answers {
        let mut deduped_group_answer: Vec<char> = group_answer.clone();
        deduped_group_answer.sort();
        deduped_group_answer.dedup();
        answer_count += deduped_group_answer.len();
    }
    println!("Part one answer: {}", answer_count);

    let mut group_answers_part2: Vec<Vec<Vec<char>>> = Vec::new();
    let mut group_answers_part2_temp: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        if line.chars().count() == 0 {
            // Appends the temp value to the full vector.
            // We do this when the count is zero because an empty line is used to separate values
            //
            //
            //
            //
            //
            //
            //
            //
            //
            //
            // WARNING!
            // Using this method means that the last line is never counted, meaning that the part one answer can be off by one.
            //
            //
            //
            //
            //
            //
            //
            //
            //
            //
            group_answers_part2.push(group_answers_part2_temp.clone());
            group_answers_part2_temp.clear();
        } else {
            // Splits the current line by whitespace and appends it to the passport_temp variable
            group_answers_part2_temp.push(line.chars().collect());
        }
    }
    let mut part2_count = 0;
    for group_answer in group_answers_part2 {
        let mut frequency_hashmap: HashMap<char, usize> = HashMap::new();
        for i in 'a'..='z' {
            frequency_hashmap.insert(i, 0);
        }
        for individual_answers in &group_answer {
            for individual_answer in individual_answers {
                let answer_count = &frequency_hashmap.get(&individual_answer).unwrap();
                frequency_hashmap.insert(*individual_answer, **answer_count + 1);
            }
        }
        part2_count += frequency_hashmap
            .iter()
            .filter(|(character, count)| **count == group_answer.len())
            .count();
    }
    println!("Part two answer: {}", part2_count);
}
