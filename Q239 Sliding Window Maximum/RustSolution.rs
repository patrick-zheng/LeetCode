use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut dq: VecDeque<usize> = VecDeque::new();
        let mut out = Vec::with_capacity(n.saturating_sub(k.saturating_sub(1)));

        for i in 0..n {
            while dq.front().is_some_and(|&j| j + k <= i) {
                dq.pop_front();
            }
            while dq.back().is_some_and(|&j| nums[j] <= nums[i]) {
                dq.pop_back();
            }
            dq.push_back(i);
            if i >= k - 1 {
                if let Some(&j) = dq.front() {
                    out.push(nums[j]);
                }
            }
        }
        out
    }
}
