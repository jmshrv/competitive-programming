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

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == 'X' {
                for vector in search_vectors {
                    if search(&input, vector, (y as i32, x as i32)) {
                        part_one += 1;
                    }
                }
            }
        }
    }

    println!("{part_one}");
}
