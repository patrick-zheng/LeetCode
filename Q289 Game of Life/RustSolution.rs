pub struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let rows = board.len();
        let cols = board[0].len();

        for r in 0..rows {
            for c in 0..cols {
                let mut live_neighbors = 0;
                for nr in r.saturating_sub(1)..=usize::min(rows - 1, r + 1) {
                    for nc in c.saturating_sub(1)..=usize::min(cols - 1, c + 1) {
                        if nr == r && nc == c {
                            continue;
                        }
                        if board[nr][nc] == 1 || board[nr][nc] == 2 {
                            live_neighbors += 1;
                        }
                    }
                }

                if board[r][c] == 1 && (live_neighbors < 2 || live_neighbors > 3) {
                    board[r][c] = 2;
                } else if board[r][c] == 0 && live_neighbors == 3 {
                    board[r][c] = 3;
                }
            }
        }

        for row in board.iter_mut() {
            for cell in row.iter_mut() {
                *cell = match *cell {
                    2 => 0,
                    3 => 1,
                    value => value,
                };
            }
        }
    }
}
