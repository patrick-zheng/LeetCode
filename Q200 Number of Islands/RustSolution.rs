pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let rows = grid.len();
        if rows == 0 {
            return 0;
        }
        let cols = grid[0].len();
        let mut islands = 0;

        fn dfs(grid: &mut Vec<Vec<char>>, r: i32, c: i32, rows: i32, cols: i32) {
            if r < 0 || c < 0 || r >= rows || c >= cols {
                return;
            }

            let (ur, uc) = (r as usize, c as usize);
            if grid[ur][uc] != '1' {
                return;
            }

            grid[ur][uc] = '0'; // mark visited

            dfs(grid, r + 1, c, rows, cols);
            dfs(grid, r - 1, c, rows, cols);
            dfs(grid, r, c + 1, rows, cols);
            dfs(grid, r, c - 1, rows, cols);
        }

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '1' {
                    islands += 1;
                    dfs(&mut grid, r as i32, c as i32, rows as i32, cols as i32);
                }
            }
        }

        islands
    }
}
