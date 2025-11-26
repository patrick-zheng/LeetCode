use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        // Put all numbers in a set for O(1) average lookup
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut longest = 0;

        // Only start counting at numbers that are the beginning of a sequence
        for &x in &set {
            if !set.contains(&(x - 1)) {
                let mut current = x;
                let mut length = 1;

                // Extend the sequence forward
                while set.contains(&(current + 1)) {
                    current += 1;
                    length += 1;
                }

                if length > longest {
                    longest = length;
                }
            }
        }

        longest
    }
}
