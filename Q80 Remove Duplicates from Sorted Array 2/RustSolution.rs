pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut w: usize = 0;
        for i in 0..nums.len() {
            let x = nums[i];
            if w < 2 || x != nums[w - 2] {
                nums[w] = x;
                w += 1;
            }
        }
        w as i32
    }
}
