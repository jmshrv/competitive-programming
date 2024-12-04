use std::io;

fn part1_search(wordsearch: &Vec<Vec<char>>, vector: (i32, i32), mut position: (i32, i32)) -> bool {
    let goal = "XMAS";
    let mut actual = "X".to_string();

    let vertical_len = wordsearch.len();
    let horizontal_len = wordsearch.first().unwrap().len();

    for _ in 0..goal.chars().count() - 1 {
        let new_y = position.0 + vector.0;
        let new_x = position.1 + vector.1;

        if new_y < 0 || new_x < 0 {
            return false;
        }

        if new_y >= vertical_len as i32 || new_x >= horizontal_len as i32 {
            return false;
        }

        position = (new_y, new_x);

        let new_char = wordsearch[new_y as usize][new_x as usize];
        actual.push(new_char);
    }

    actual == goal
}

fn part2_search(wordsearch: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    let vertical_len = wordsearch.len();
    let horizontal_len = wordsearch.first().unwrap().len();

    if position.0 == 0 || position.1 == 0 {
        return false;
    }

    if position.0 >= vertical_len - 1 || position.1 >= horizontal_len - 1 {
        return false;
    }

    let top_left = wordsearch[position.0 - 1][position.1 - 1];
    let top_right = wordsearch[position.0 - 1][position.1 + 1];
    let bottom_left = wordsearch[position.0 + 1][position.1 - 1];
    let bottom_right = wordsearch[position.0 + 1][position.1 + 1];

    let valid_perms = [
        ['M', 'S', 'S', 'M'],
        ['M', 'M', 'S', 'S'],
        ['S', 'M', 'M', 'S'],
        ['S', 'S', 'M', 'M'],
    ];

    valid_perms.contains(&[top_left, top_right, bottom_right, bottom_left])
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let search_vectors = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut part_one = 0;
    let mut part_two = 0;

    let xes = input
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .filter(|(_, char)| **char == 'X')
                .collect::<Vec<_>>()
        })
        .filter(|line| !line.is_empty());

    for (y, line) in xes.enumerate() {
        for (x, _) in line {
            part_one += search_vectors
                .iter()
                .filter(|vector| part1_search(&input, **vector, (y as i32, x as i32)))
                .count();
        }
    }

    println!("{part_one}");

    let cross_middles = input
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .filter(|(_, char)| **char == 'A')
                .collect::<Vec<_>>()
        })
        .filter(|line| !line.is_empty());

    for (y, line) in cross_middles.enumerate() {
        for (x, _) in line {
            if part2_search(&input, (y, x)) {
                part_two += 1;
            }
        }
    }

    println!("{part_two}");
}
