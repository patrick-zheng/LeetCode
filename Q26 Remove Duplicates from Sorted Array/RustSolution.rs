pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut write = 0;
        for &read in nums.clone().iter() {
            if write == 0 || read != nums[write - 1] {
                nums[write] = read;
                write += 1;
            }
        }
        write as i32
    }
}

