pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_ending = nums[0];
        let mut min_ending = nums[0];
        let mut best = nums[0];

        for &x in nums.iter().skip(1) {
            // Negative flips roles of max/min
            if x < 0 {
                std::mem::swap(&mut max_ending, &mut min_ending);
            }

            max_ending = std::cmp::max(x, x * max_ending);
            min_ending = std::cmp::min(x, x * min_ending);

            best = std::cmp::max(best, max_ending);
        }

        best
    }
}
