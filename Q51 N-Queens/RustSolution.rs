pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn backtrack(
            row: i32,
            n: i32,
            cols: i32,
            diag1: i32,
            diag2: i32,
            board: &mut Vec<usize>,
            result: &mut Vec<Vec<String>>,
        ) {
            if row == n {
                let mut solution = Vec::with_capacity(n as usize);
                for &q_col in board.iter() {
                    let mut row_str = String::with_capacity(n as usize);
                    for c in 0..n {
                        if c as usize == q_col {
                            row_str.push('Q');
                        } else {
                            row_str.push('.');
                        }
                    }
                    solution.push(row_str);
                }
                result.push(solution);
                return;
            }

            let mut available = ((1 << n) - 1) & !(cols | diag1 | diag2);
            while available != 0 {
                let bit = available & -available;
                available -= bit;
                let col = bit.trailing_zeros() as usize;

                board[row as usize] = col;
                backtrack(
                    row + 1,
                    n,
                    cols | bit,
                    (diag1 | bit) << 1,
                    (diag2 | bit) >> 1,
                    board,
                    result,
                );
            }
        }

        let mut result = Vec::new();
        let mut board = vec![0; n as usize];
        backtrack(0, n, 0, 0, 0, &mut board, &mut result);
        result
    }
}
