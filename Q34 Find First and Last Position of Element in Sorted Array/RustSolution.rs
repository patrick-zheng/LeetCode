pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        fn find_left_index(nums: &Vec<i32>, target: i32) -> usize {
            let (mut left, mut right) = (0, nums.len());
            while left < right {
                let mid = (left + right) / 2;
                if nums[mid] < target {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            left
        }

        fn find_right_index(nums: &Vec<i32>, target: i32) -> usize {
            let (mut left, mut right) = (0, nums.len());
            while left < right {
                let mid = (left + right) / 2;
                if nums[mid] <= target {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            if left == 0 { 0 } else { left - 1 }
        }

        let left = find_left_index(&nums, target);
        let right = find_right_index(&nums, target);

        if left <= right && right < nums.len() && nums[left] == target && nums[right] == target {
            vec![left as i32, right as i32]
        } else {
            vec![-1, -1]
        }
    }
}
