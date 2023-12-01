use std::fs;

fn calculate_hits(right_by: usize, down_by: usize) -> usize {
    let map_input = fs::read_to_string("src/input.txt").expect("Unable to read file");

    let extend_by = right_by * map_input.lines().count();
    let mut extended_map = String::new();

    // Extend the map by EXTEND_BYx to ensure that we have enough space to go right without hitting the end of the line
    for line in map_input.lines() {
        extended_map = format!("{}{}\n", extended_map, line.repeat(extend_by));
    }

    let mut x_position = 0;
    let mut trees_hit = 0;

    // Iterate over the extended map (stepped by down_by, which is essentially what interval the y position moves at) and check if there is a tree at x_position (if x_position is 0, the first character is checked, if x_position is 2, the 3rd character is checked etc).
    // right_by is added to x_position every iteration to move diagonally.
    for line in extended_map.lines().step_by(down_by) {
        let exact_position = line.chars().nth(x_position).unwrap();
        if exact_position == '#' {
            trees_hit += 1;
        }
        x_position += right_by;
    }

    return trees_hit;
}

fn main() {
    // PART 1
    println!("{}", calculate_hits(3, 1));

    // PART 2
    let mut results = vec![];

    results.insert(results.len(), calculate_hits(1, 1));
    results.insert(results.len(), calculate_hits(3, 1));
    results.insert(results.len(), calculate_hits(5, 1));
    results.insert(results.len(), calculate_hits(7, 1));
    results.insert(results.len(), calculate_hits(1, 2));
    let mut answer = 0;

    for result in results {
        if answer == 0 {
            answer = result;
        } else {
            answer *= result;
        }
    }

    println!("{}", answer);
}
