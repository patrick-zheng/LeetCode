# 🧩 LeetCode Problem: Subsets

- **Problem Link**: [Subsets – LeetCode](https://leetcode.com/problems/subsets/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/subsets/solutions/)

---

## 🧠 Algorithm Explanation

We are asked to return all possible subsets (the power set) of a given array of unique integers.  

There are `2^n` possible subsets, since each element can either be **included** or **excluded** from a subset.  

A natural way to solve this is **backtracking (DFS recursion)**:

- At each index, we decide whether to include the current element or skip it.
- When we reach the end of the array, we add the current subset to the results.

This approach ensures all subsets are generated without duplicates.

---

### 🪜 Steps

1. **Start recursion** with an empty subset at index `0`.
2. **At each index**, branch into two choices:
   - Exclude the current element.
   - Include the current element.
3. **Base case**: when the index reaches the length of the array, add the current subset to the results.

---

## ✅ Constraints

- `1 <= nums.length <= 10`
- `-10 <= nums[i] <= 10`
- All elements are unique.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(2^n * n) |
| Space Complexity  | O(2^n * n) |

---
