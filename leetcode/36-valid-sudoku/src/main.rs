use std::collections::HashSet;

struct Solution {}

impl Solution {
    fn is_segment_valid<'a>(segment: impl Iterator<Item = &'a char> + Clone) -> bool {
        let numbers_only_segment = segment.filter(|&&c| c != '.');

        numbers_only_segment.clone().collect::<HashSet<_>>().len() == numbers_only_segment.count()
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let rows_valid = board.iter().all(|row| Self::is_segment_valid(row.iter()));

        if !rows_valid {
            return false;
        }

        let board_width = board.first().expect("Empty board!").len();

        let columns_valid = (0..board_width)
            .all(|column_index| Self::is_segment_valid(board.iter().map(|row| &row[column_index])));

        if !columns_valid {
            return false;
        }

        for vertical_chunk in 0..3 {
            for horizontal_chunk in 0..3 {
                let starty = vertical_chunk * 3;
                let startx = horizontal_chunk * 3;

                let mut chunk = vec![];

                for y in starty..starty + 3 {
                    for x in startx..startx + 3 {
                        chunk.push(board[y][x]);
                    }
                }

                if !Self::is_segment_valid(chunk.iter()) {
                    return false;
                }
            }
        }

        true
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

    let invalid = Solution::is_valid_sudoku(vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ]);

    println!("{valid}");
    println!("{invalid}");
}
