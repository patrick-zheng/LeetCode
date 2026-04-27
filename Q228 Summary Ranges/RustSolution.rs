pub struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return Vec::new();
        }

        let mut ranges: Vec<String> = Vec::new();
        let mut start = nums[0];
        let mut prev = nums[0];

        for &num in nums.iter().skip(1) {
            if num == prev + 1 {
                prev = num;
                continue;
            }

            if start == prev {
                ranges.push(start.to_string());
            } else {
                ranges.push(format!("{start}->{prev}"));
            }

            start = num;
            prev = num;
        }

        if start == prev {
            ranges.push(start.to_string());
        } else {
            ranges.push(format!("{start}->{prev}"));
        }

        ranges
    }
}
