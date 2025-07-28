# 🧩 LeetCode Problem: Combination Sum II

- **Problem Link**: [Combination Sum II – LeetCode](https://leetcode.com/problems/combination-sum-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/combination-sum-ii/solutions/)

---

## 🧠 Algorithm Explanation

We use **backtracking (DFS)** to explore all combinations that sum up to the target.  
To ensure **uniqueness of combinations**, we sort the input and **skip duplicates** while exploring candidates.  
Each candidate is used **only once** per combination (unlike `Combination Sum I` where reuse is allowed).

Sorting enables two major optimizations:

- Prune branches early when current sum exceeds target.
- Efficiently skip over duplicate elements to prevent repeated combinations.

---

### 🪜 Steps

1. **Sort the candidates array** to enable duplicate skipping and pruning.
2. **Initialize a recursive backtracking function** with parameters:
   - `start`: index to start exploring from
   - `path`: current combination being built
   - `total`: current sum of `path`
3. **At each recursive call**:
   - If `total == target`, store the current path as a valid result.
   - If `total > target`, stop exploring that path (prune).
   - Loop through candidates from `start` to end:
     - Skip duplicates (`if i > start and candidates[i] == candidates[i-1]`)
     - If adding `candidates[i]` keeps total within target, explore further.
     - Recurse with `i + 1` (move forward to avoid reusing same element)
     - Backtrack by removing the last added number.

---

## ✅ Constraints

- `1 <= candidates.length <= 100`
- `1 <= candidates[i] <= 50`
- `1 <= target <= 30`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(2^n)     |
| Space Complexity  | O(n)       |

> Where `n` is the number of candidates. Sorting costs `O(n log n)`, and the recursive stack can go up to depth `n`.

---
