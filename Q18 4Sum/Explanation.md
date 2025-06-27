# 🧩 LeetCode Problem: 4Sum

- **Problem Link**: [4Sum – LeetCode](https://leetcode.com/problems/4sum/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/4sum/solutions/)

---

## 🧠 Algorithm Explanation

We use a combination of **sorting** and a **two-pointer approach** to efficiently find all unique quadruplets that sum up to the given target. The array is sorted first to help skip duplicates and apply early termination conditions. Then, we fix the first two numbers using nested loops and find the remaining two numbers using two pointers (`left` and `right`) within the rest of the array.

This algorithm is chosen because:
- It avoids brute force O(n⁴) complexity.
- It efficiently skips duplicate combinations.
- It takes advantage of the sorted order to prune unnecessary checks.

---

### 🪜 Steps

1. **Sort the array** to simplify duplicate handling and enable two-pointer technique.
2. **Iterate with two nested loops** to fix the first two numbers of the quadruplet.
3. **Apply two-pointer approach** to find the remaining two numbers that make up the target sum.
4. **Skip duplicates** at each level to ensure uniqueness.
5. **Apply early termination** if it's impossible to reach the target with current selections.

---

## ✅ Constraints

- `1 <= nums.length <= 200`
- `-10⁹ <= nums[i] <= 10⁹`
- `-10⁹ <= target <= 10⁹`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n³)      |
| Space Complexity  | O(1)       | (excluding output list)

---
