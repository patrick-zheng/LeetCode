pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return vec![];
        }

        let (mut top, mut bottom) = (0, matrix.len() as i32 - 1);
        let (mut left, mut right) = (0, matrix[0].len() as i32 - 1);
        let mut result = Vec::with_capacity(matrix.len() * matrix[0].len());

        while top <= bottom && left <= right {
            // Left → Right
            for i in left..=right {
                result.push(matrix[top as usize][i as usize]);
            }
            top += 1;

            // Top → Bottom
            for i in top..=bottom {
                result.push(matrix[i as usize][right as usize]);
            }
            right -= 1;

            // Right → Left
            if top <= bottom {
                for i in (left..=right).rev() {
                    result.push(matrix[bottom as usize][i as usize]);
                }
                bottom -= 1;
            }

            // Bottom → Top
            if left <= right {
                for i in (top..=bottom).rev() {
                    result.push(matrix[i as usize][left as usize]);
                }
                left += 1;
            }
        }

        result
    }
}

