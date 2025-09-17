# 🧩 LeetCode Problem: Combinations

- **Problem Link**: [Combinations – LeetCode](https://leetcode.com/problems/combinations/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/combinations/solutions/)

---

## 🧠 Algorithm Explanation

We need to generate all possible **k-element combinations** chosen from the range `[1, n]`.  
This is a classic **backtracking** problem, where we build solutions step by step:

- Use recursion (DFS) to explore choices.
- Maintain a temporary `path` to hold the current combination.
- Once `path` has `k` elements, add it to the results.
- Apply **pruning** so that we do not explore branches that cannot possibly yield valid results (e.g., when not enough numbers remain to complete the combination).

This algorithm is chosen because backtracking systematically explores all possible subsets while avoiding unnecessary work.

---

### 🪜 Steps

1. **Initialize**: Start with an empty list `path` and an empty result list `res`.  
2. **Recursive DFS**:
   - If `path` has `k` numbers, add a copy to `res`.  
   - Otherwise, iterate through numbers from the current `start` up to `n`, adding each to `path` and recursing with `start + 1`.  
3. **Backtrack**: Remove the last element after recursion to explore other branches.  

---

## ✅ Constraints

- `1 <= n <= 20`
- `1 <= k <= n`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | \(O(\binom{n}{k} \cdot k)\) |
| Space Complexity  | \(O(k)\) recursion depth (excluding output) |

---
