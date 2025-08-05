# 🧩 LeetCode Problem: Permutations

- **Problem Link**: [Permutations – LeetCode](https://leetcode.com/problems/permutations/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/permutations/solutions/)

---

## 🧠 Algorithm Explanation

We need to generate all possible permutations of the given list of distinct integers.  
Instead of manually implementing a backtracking algorithm, we can use Python's built-in `itertools.permutations`, which is highly optimized in C and provides all possible permutations in lexicographic order as tuples.  
We simply convert these tuples into lists.

---

### 🪜 Steps

1. **Import `itertools.permutations`**.
2. **Call `permutations(nums)`** to generate all possible permutations of the array.
3. **Convert each permutation tuple into a list** since the problem requires lists, not tuples.
4. **Return the list of all permutations**.

---

## ✅ Constraints

- `1 <= nums.length <= 6`
- `-10 <= nums[i] <= 10`
- All the integers of `nums` are **unique**.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity        |
|-------------------|-------------------|
| Time Complexity   | O(n × n!)         |
| Space Complexity  | O(n × n!)         |

---
