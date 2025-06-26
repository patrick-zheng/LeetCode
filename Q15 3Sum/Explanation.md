# 🧩 LeetCode Problem: 3Sum

- **Problem Link**: [3Sum – LeetCode](https://leetcode.com/problems/3sum/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/3sum/solutions/)

---

## 🧠 Algorithm Explanation

The goal is to find all unique triplets in the array which sum up to zero. A brute-force approach would be to try all combinations using three nested loops, but this would result in O(n³) time complexity. Instead, we can use a **sorting + two-pointer approach** to reduce the time complexity to O(n²).

We first **sort the array**, which allows us to use the two-pointer technique and efficiently skip duplicates. Then for each number, we fix it and look for two other numbers that sum to the negative of the fixed number.

This approach avoids duplicate triplets and ensures optimal performance.

---

### 🪜 Steps

1. **Sort the array**: This helps in avoiding duplicates and efficiently finding pairs using two pointers.
2. **Iterate through the array with index `i`**:
   - Skip duplicates: if `nums[i] == nums[i - 1]`, continue.
   - Set `left = i + 1` and `right = len(nums) - 1`.
3. **While `left < right`**:
   - Calculate the sum `total = nums[i] + nums[left] + nums[right]`.
   - If `total == 0`, store the triplet, and move both pointers while skipping duplicates.
   - If `total < 0`, increment `left` to increase the sum.
   - If `total > 0`, decrement `right` to decrease the sum.

---

## ✅ Constraints

- `3 <= nums.length <= 3000`
- `-10⁵ <= nums[i] <= 10⁵`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n²)      |
| Space Complexity  | O(1) (excluding output) |

---
