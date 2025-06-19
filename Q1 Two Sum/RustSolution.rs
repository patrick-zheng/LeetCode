/*
 * Problem: https://leetcode.com/problems/two-sum/
 * Solution: https://leetcode.com/problems/two-sum/solutions/
 * Time Complexity: Brute Force O(n^2), Sorted Array with Two Pointers O(n log n), Hash Map Lookup O(n)
 * Space Complexity: O(n)
 */

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum_brute_force(nums: &[i32], target: i32) -> Vec<usize> {
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i, j];
                }
            }
        }
        vec![]
    }

    pub fn two_sum_sorted_two_pointers(nums: &[i32], target: i32) -> Vec<usize> {
        let mut indexed_nums: Vec<(usize, i32)> = nums.iter().cloned().enumerate().collect();
        indexed_nums.sort_by_key(|&(_, num)| num);

        let (mut left, mut right) = (0, indexed_nums.len() - 1);

        while left < right {
            let sum = indexed_nums[left].1 + indexed_nums[right].1;
            if sum == target {
                return vec![indexed_nums[left].0, indexed_nums[right].0];
            }
            else if sum < target { left += 1; }
            else { right -= 1; }
        }

        vec![]
    }

    pub fn two_sum_hash_map(nums: &[i32], target: i32) -> Vec<usize> {
        let mut num_to_index = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&idx) = num_to_index.get(&complement) {
                return vec![idx, i];
            }
            num_to_index.entry(num).or_insert(i);
        }

        vec![]
    }

    pub fn solve(nums: &[i32], target: i32, method: &str) -> Vec<usize> {
        match method {
            "bruteForce" => Self::two_sum_brute_force(nums, target),
            "sortedArrayTwoPointers" => Self::two_sum_sorted_two_pointers(nums, target),
            "hashMapLookup" => Self::two_sum_hash_map(nums, target),
            _ => panic!("Unknown method"),
        }
    }
}