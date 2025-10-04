# 🧩 LeetCode Problem: Subsets II

- **Problem Link**: [Subsets II – LeetCode](https://leetcode.com/problems/subsets-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/subsets-ii/solutions/)

---

## 🧠 Algorithm Explanation

The problem asks for generating all possible subsets (the power set) of an array of integers that **may contain duplicates**. The key requirement is to **avoid duplicate subsets** in the final result.

The algorithm uses **backtracking**:

- First, the array is sorted to ensure that duplicates are adjacent.
- At each step, we recursively build subsets by choosing whether to include each element.
- To avoid duplicate subsets, we **skip duplicate numbers** when they appear consecutively at the same recursion depth.

This ensures that all subsets are unique and that every possible subset is explored.

---

### 🪜 Steps

1. **Sort the array**: This ensures that duplicates are next to each other, making them easier to detect and skip.
2. **Backtracking function**:
   - Add the current path (subset under construction) to the result.
   - Iterate over the remaining elements:
     - If the current element is the same as the previous one **and** we are at the same recursion level, skip it.
     - Otherwise, recurse with the current element included in the path.
3. **Return the result** after exploring all possibilities.

---

## ✅ Constraints

- `1 <= nums.length <= 10`
- `-10 <= nums[i] <= 10`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(2^n)** in the worst case (without duplicates). Sorting adds **O(n log n)**, but the dominant term is the exponential subset generation. Duplicate skipping helps reduce redundant work. |
| Space Complexity  | **O(n)** for recursion stack depth, plus storage for results. |

---
