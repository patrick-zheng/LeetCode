use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        if board.is_empty() || board[0].is_empty() || word.is_empty() {
            return false;
        }

        // Make board mutable
        let mut board = board;
        let rows = board.len();
        let cols = board[0].len();

        // --- Optimization 1: letter-count pruning ---
        let mut bcnt: HashMap<char, usize> = HashMap::new();
        for row in &board {
            for &ch in row {
                *bcnt.entry(ch).or_insert(0) += 1;
            }
        }

        let mut wchars: Vec<char> = word.chars().collect();
        let mut wcnt: HashMap<char, usize> = HashMap::new();
        for &ch in &wchars {
            *wcnt.entry(ch).or_insert(0) += 1;
        }
        for (ch, cnt) in wcnt.iter() {
            if bcnt.get(ch).copied().unwrap_or(0) < *cnt {
                return false;
            }
        }

        // --- Optimization 2: search from the rarer end ---
        let first = wchars[0];
        let last = *wchars.last().unwrap();
        if bcnt.get(&first).unwrap_or(&0) > bcnt.get(&last).unwrap_or(&0) {
            wchars.reverse();
        }

        fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize, w: &Vec<char>, i: usize) -> bool {
            if board[r][c] != w[i] {
                return false;
            }
            if i == w.len() - 1 {
                return true;
            }

            let tmp = board[r][c];
            board[r][c] = '#'; // mark visited

            let dirs: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            let rows = board.len() as isize;
            let cols = board[0].len() as isize;

            for (dr, dc) in dirs {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                    let ur = nr as usize;
                    let uc = nc as usize;
                    if board[ur][uc] != '#' && dfs(board, ur, uc, w, i + 1) {
                        board[r][c] = tmp; // restore before returning
                        return true;
                    }
                }
            }

            board[r][c] = tmp; // restore
            false
        }

        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == wchars[0] && dfs(&mut board, i, j, &wchars, 0) {
                    return true;
                }
            }
        }
        false
    }
}
