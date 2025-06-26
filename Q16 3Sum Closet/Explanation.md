# 🧩 LeetCode Problem: 3Sum Closest

- **Problem Link**: [3Sum Closest – LeetCode](https://leetcode.com/problems/3sum-closest/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/3sum-closest/solutions/)

---

## 🧠 Algorithm Explanation

The goal is to find the sum of a triplet in the array that is closest to the given target. We use the **two-pointer technique** combined with **sorting** to achieve an efficient solution.

By fixing one number and using two pointers to scan through the rest of the array, we can evaluate potential triplets efficiently without checking every combination.

---

### 🪜 Steps

1. **Sort the array** in ascending order.
2. **Iterate** through the array from index `i = 0` to `n - 3`:
   - Fix `nums[i]` as the first number in the triplet.
   - Initialize two pointers: `left = i + 1`, `right = n - 1`.
3. **While `left < right`**:
   - Calculate the sum: `current_sum = nums[i] + nums[left] + nums[right]`
   - If `abs(current_sum - target)` is smaller than the current closest distance, update the closest sum.
   - If `current_sum < target`, move `left` to the right.
   - If `current_sum > target`, move `right` to the left.
   - If `current_sum == target`, return the sum immediately.

---

## ✅ Constraints

- `3 <= nums.length <= 500`
- `-1000 <= nums[i] <= 1000`
- `-10⁴ <= target <= 10⁴`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n²)      |
| Space Complexity  | O(1)       |

> Sorting takes O(n log n), and the two-pointer traversal for each element gives an overall O(n²) complexity.

---
