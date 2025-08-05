# 🧩 LeetCode Problem: Permutations II

- **Problem Link**: [Permutations II – LeetCode](https://leetcode.com/problems/permutations-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/permutations-ii/solutions/)

---

## 🧠 Algorithm Explanation

The goal is to return all the **unique permutations** of a given list of integers `nums` which may contain duplicates.

This solution uses Python’s built-in `itertools.permutations` function to generate all possible permutations (including duplicates), and then uses a `set` to automatically remove any duplicates.

Although simple to write and conceptually easy, this approach is **not optimal** in terms of performance, especially when `nums` contains many duplicate values. All `n!` permutations are generated first, which can consume a lot of memory and computation time, and then filtered afterward.

---

### 🪜 Steps

1. **Generate all permutations**:  
   Use `itertools.permutations(nums)` to get all possible orderings (as tuples).

2. **Remove duplicates**:  
   Convert the list of permutations to a `set`, which removes any duplicate tuples.

3. **Convert back to lists**:  
   Since the problem expects `List[List[int]]`, convert each tuple back to a list.

---

## ✅ Constraints

- `1 <= nums.length <= 8`
- `-10 <= nums[i] <= 10`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity    |
|-------------------|---------------|
| Time Complexity   | O(n! × n)     |
| Space Complexity  | O(n! × n)     |

- `n!`: Total number of permutations.
- Each permutation takes O(n) space, and we store all of them before deduplicating.

---
