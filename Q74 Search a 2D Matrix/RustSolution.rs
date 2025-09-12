pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() { return false; }
        let m = matrix.len();
        let n = matrix[0].len();
        let (mut lo, mut hi) = (0_i32, (m * n - 1) as i32);

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let r = (mid as usize) / n;
            let c = (mid as usize) % n;
            let val = matrix[r][c];

            if val == target { return true; }
            if val < target { lo = mid + 1; } else { hi = mid - 1; }
        }
        false
    }
}
