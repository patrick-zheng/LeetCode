use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        if m == 0 {
            return;
        }
        let n = board[0].len();
        if n == 0 {
            return;
        }

        fn bfs(board: &mut Vec<Vec<char>>, start_r: usize, start_c: usize) {
            let m = board.len();
            let n = board[0].len();
            let mut queue = VecDeque::new();
            board[start_r][start_c] = 'E';
            queue.push_back((start_r, start_c));

            let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            while let Some((x, y)) = queue.pop_front() {
                for (dx, dy) in dirs.iter() {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && nx < m as isize && ny >= 0 && ny < n as isize {
                        let (ux, uy) = (nx as usize, ny as usize);
                        if board[ux][uy] == 'O' {
                            board[ux][uy] = 'E';
                            queue.push_back((ux, uy));
                        }
                    }
                }
            }
        }

        // 1. Border BFS for 'O' cells
        for i in 0..m {
            if board[i][0] == 'O' {
                bfs(board, i, 0);
            }
            if board[i][n - 1] == 'O' {
                bfs(board, i, n - 1);
            }
        }

        for j in 0..n {
            if board[0][j] == 'O' {
                bfs(board, 0, j);
            }
            if board[m - 1][j] == 'O' {
                bfs(board, m - 1, j);
            }
        }

        // 2. Flip captured and restore escaped
        for i in 0..m {
            for j in 0..n {
                match board[i][j] {
                    'O' => board[i][j] = 'X',
                    'E' => board[i][j] = 'O',
                    _ => {}
                }
            }
        }
    }
}
