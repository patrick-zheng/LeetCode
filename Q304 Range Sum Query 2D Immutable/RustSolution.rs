pub struct NumMatrix {
    prefix: Vec<Vec<i32>>,
}

impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let rows = matrix.len();
        let cols = matrix.first().map_or(0, |row| row.len());
        let mut prefix = vec![vec![0; cols + 1]; rows + 1];

        for r in 0..rows {
            for c in 0..cols {
                prefix[r + 1][c + 1] = matrix[r][c] + prefix[r][c + 1]
                    + prefix[r + 1][c]
                    - prefix[r][c];
            }
        }

        Self { prefix }
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let r1 = row1 as usize;
        let c1 = col1 as usize;
        let r2 = row2 as usize;
        let c2 = col2 as usize;
        self.prefix[r2 + 1][c2 + 1] - self.prefix[r1][c2 + 1]
            - self.prefix[r2 + 1][c1]
            + self.prefix[r1][c1]
    }
}
