pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = {
            let mut cloned = nums.clone();
            cloned.sort();
            cloned
        };

        let n = nums.len();
        let mut res = Vec::new();

        for i in 0..n - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            if nums[i] as i64 + nums[i + 1] as i64 + nums[i + 2] as i64 + nums[i + 3] as i64 > target as i64 {
                break;
            }

            if nums[i] as i64 + nums[n - 3] as i64 + nums[n - 2] as i64 + nums[n - 1] as i64 < target as i64 {
                continue;
            }

            for j in i + 1..n - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }

                if nums[i] as i64 + nums[j] as i64 + nums[j + 1] as i64 + nums[j + 2] as i64 > target as i64 {
                    break;
                }

                if nums[i] as i64 + nums[j] as i64 + nums[n - 2] as i64 + nums[n - 1] as i64 < target as i64 {
                    continue;
                }

                let (mut left, mut right) = (j + 1, n - 1);
                while left < right {
                    let sum = nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;

                    if sum == target as i64 {
                        res.push(vec![nums[i], nums[j], nums[left], nums[right]]);

                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }

                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }

                        left += 1;
                        right -= 1;
                    } else if sum < target as i64 {
                        left += 1;
                    } else {
                        right -= 1;
                    }
                }
            }
        }

        res
    }
}

