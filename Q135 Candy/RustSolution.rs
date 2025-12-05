pub struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        if n == 0 {
            return 0;
        }

        let mut left = vec![1; n];
        let mut right = vec![1; n];

        // Left to right
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                left[i] = left[i - 1] + 1;
            }
        }

        // Right to left
        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                right[i] = right[i + 1] + 1;
            }
        }

        let mut total = 0;
        for i in 0..n {
            total += left[i].max(right[i]);
        }

        total
    }
}
