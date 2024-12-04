use std::io;

fn search(wordsearch: &Vec<Vec<char>>, vector: (i32, i32), mut position: (i32, i32)) -> bool {
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

    let xes = input
        .iter()
        .enumerate()
        .map(|(_, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, char)| **char == 'X')
                .collect::<Vec<_>>()
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    for (y, line) in xes.iter().enumerate() {
        for (x, _) in line {
            part_one += search_vectors
                .iter()
                .filter(|vector| search(&input, **vector, (y as i32, *x as i32)))
                .count();
        }
    }

    println!("{part_one}");
}
