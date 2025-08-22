pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n_usize = n as usize;
        let mut matrix = vec![vec![0; n_usize]; n_usize];

        let (mut left, mut right, mut top, mut bottom) = (0i32, n - 1, 0i32, n - 1);
        let mut num = 1;

        while left <= right && top <= bottom {
            // top row: left -> right
            for c in left..=right {
                matrix[top as usize][c as usize] = num;
                num += 1;
            }
            top += 1;

            // right column: top -> bottom
            for r in top..=bottom {
                matrix[r as usize][right as usize] = num;
                num += 1;
            }
            right -= 1;

            // bottom row: right -> left
            if top <= bottom {
                for c in (left..=right).rev() {
                    matrix[bottom as usize][c as usize] = num;
                    num += 1;
                }
                bottom -= 1;
            }

            // left column: bottom -> top
            if left <= right {
                for r in (top..=bottom).rev() {
                    matrix[r as usize][left as usize] = num;
                    num += 1;
                }
                left += 1;
            }
        }

        matrix
    }
}
