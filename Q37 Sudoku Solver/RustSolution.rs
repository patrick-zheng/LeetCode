pub struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) -> void {
        let mut rows = [[false; 9]; 9];
        let mut cols = [[false; 9]; 9];
        let mut boxes = [[false; 9]; 9];
        let mut empty = vec![];

        for r in 0..9 {
            for c in 0..9 {
                let ch = board[r][c];
                if ch == '.' {
                    empty.push((r, c));
                } else {
                    let d = (ch as u8 - b'1') as usize;
                    rows[r][d] = true;
                    cols[c][d] = true;
                    boxes[(r / 3) * 3 + (c / 3)][d] = true;
                }
            }
        }

        fn backtrack(
            idx: usize,
            board: &mut Vec<Vec<char>>,
            empty: &[(usize, usize)],
            rows: &mut [[bool; 9]; 9],
            cols: &mut [[bool; 9]; 9],
            boxes: &mut [[bool; 9]; 9],
        ) -> bool {
            if idx == empty.len() {
                return true;
            }

            let (r, c) = empty[idx];
            let b = (r / 3) * 3 + (c / 3);

            for d in 0..9 {
                if !rows[r][d] && !cols[c][d] && !boxes[b][d] {
                    board[r][c] = (b'1' + d as u8) as char;
                    rows[r][d] = true;
                    cols[c][d] = true;
                    boxes[b][d] = true;

                    if backtrack(idx + 1, board, empty, rows, cols, boxes) {
                        return true;
                    }

                    board[r][c] = '.';
                    rows[r][d] = false;
                    cols[c][d] = false;
                    boxes[b][d] = false;
                }
            }

            false
        }

        backtrack(0, board, &empty, &mut rows, &mut cols, &mut boxes);
    }
}

