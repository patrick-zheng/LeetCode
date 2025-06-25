pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let mut max_volume = 0;

        while left < right {
            let hl = height[left];
            let hr = height[right];
            let width = (right - left) as i32;

            let volume = hl.min(hr) * width;
            if volume > max_volume { max_volume = volume; }

            if hl < hr { left += 1; }
            else { right -= 1; }
        }

        max_volume
    }
}

