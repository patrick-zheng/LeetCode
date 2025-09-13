pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.is_empty() {
            return;
        }

        let (mut low, mut mid): (usize, usize) = (0, 0);
        let mut high: isize = nums.len() as isize - 1; // use isize for safe decrement checks

        while (mid as isize) <= high {
            match nums[mid] {
                0 => {
                    nums.swap(low, mid);
                    low += 1;
                    mid += 1;
                }
                1 => {
                    mid += 1;
                }
                _ => {
                    nums.swap(mid, high as usize);
                    high -= 1;
                    // do not increment mid here; need to re-check the swapped-in value
                }
            }
        }
    }
}
