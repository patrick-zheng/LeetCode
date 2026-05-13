pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut r = 0usize;
        let mut c = cols as i32 - 1;

        while r < rows && c >= 0 {
            let v = matrix[r][c as usize];
            if v == target {
                return true;
            }
            if v > target {
                c -= 1;
            } else {
                r += 1;
            }
        }
        false
    }
}
