pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut write = 0usize;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                if i != write {
                    nums.swap(write, i);
                }
                write += 1;
            }
        }
    }
}
