# 🧩 LeetCode Problem: Combination Sum

- **Problem Link**: [Combination Sum – LeetCode](https://leetcode.com/problems/combination-sum/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/combination-sum/solutions/)

---

## 🧠 Algorithm Explanation

This solution uses **backtracking** to explore all combinations of numbers that sum up to the `target`. It tries each candidate starting from the current index (`start`) so that the same number can be reused multiple times.

If the remaining target becomes negative, the path is invalid and is discarded. If the remaining target is `0`, a valid combination is found and added to the result.

Backtracking is suitable because:

- We're incrementally building combinations.
- We want to explore all valid paths.
- We prune (terminate) invalid paths early to avoid unnecessary computation.

---

### 🪜 Steps

1. **Initialize** an empty list `result` to store valid combinations.
2. **Define a recursive function `backtrack(start, path, remaining)`**:
   - If `remaining == 0`, append a copy of `path` to `result`.
   - If `remaining < 0`, terminate the current path.
   - Loop through candidates starting from index `start`:
     - Append `candidates[i]` to `path`, and call `backtrack(i, path, remaining - candidates[i])`.
3. **Invoke `backtrack(0, [], target)`** to start.
4. **Return `result`**.

---

## ✅ Constraints

- `1 <= candidates.length <= 30`
- `2 <= candidates[i] <= 40`
- All elements of `candidates` are **distinct**.
- `1 <= target <= 40`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity        |
|-------------------|-------------------|
| Time Complexity   | O(2^T)            |
| Space Complexity  | O(T)              |

- **Time Complexity**: In the worst case, exponential in the target value `T`, due to multiple branching recursive calls.
- **Space Complexity**: O(T) due to the recursion stack depth and path storage.

---
