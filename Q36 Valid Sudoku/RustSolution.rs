use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![HashSet::new(); 9];
        let mut cols = vec![HashSet::new(); 9];
        let mut boxes = vec![HashSet::new(); 9];

        for r in 0..9 {
            for c in 0..9 {
                let num = board[r][c];
                if num == '.' {
                    continue;
                }

                let box_index = (r / 3) * 3 + (c / 3);
                if rows[r].contains(&num) || cols[c].contains(&num) || boxes[box_index].contains(&num) {
                    return false;
                }

                rows[r].insert(num);
                cols[c].insert(num);
                boxes[box_index].insert(num);
            }
        }

        true
    }
}
