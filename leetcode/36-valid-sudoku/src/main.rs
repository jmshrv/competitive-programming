use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for row in &board {
            if !Solution::is_line_unique(&row) {
                return false;
            }
        }

        for column_index in 0..board.first().unwrap().len() {
            let column = board
                .iter()
                .map(|row| row[column_index])
                .collect::<Vec<_>>();

            if !Solution::is_line_unique(&column) {
                return false;
            }
        }

        let sliced_board = board
            .iter()
            .map(|row| row.chunks_exact(3).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for rows in sliced_board.chunks_exact(3) {
            let chunked_rows = rows
                .iter()
                .map(|row| row.chunks_exact(3).collect::<Vec<_>>())
                .collect::<Vec<_>>();

            for i in 0..3 {
                let square = chunked_rows
                    .iter()
                    .map(|chunk| chunk[i])
                    .flatten()
                    .collect::<Vec<_>>();

                println!("{square:?}");
            }
        }
        true
    }

    fn is_line_unique(line: &[char]) -> bool {
        let empty_spaces = line.iter().filter(|c| **c == '.').count();
        let unique_count = line
            .iter()
            .filter(|c| **c != '.')
            .collect::<HashSet<_>>()
            .len();

        line.len() == empty_spaces + unique_count
    }
}

fn main() {
    let valid = Solution::is_valid_sudoku(vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ]);

    // let invalid = Solution::is_valid_sudoku(vec![
    //     vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
    //     vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
    //     vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
    //     vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
    //     vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
    //     vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
    //     vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
    //     vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
    //     vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    // ]);

    println!("{valid}");
    // println!("{invalid}");
}
