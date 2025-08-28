# 🧩 LeetCode Problem: Unique Paths II

- **Problem Link**: [Unique Paths II – LeetCode](https://leetcode.com/problems/unique-paths-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/unique-paths-ii/solutions/)

---

## 🧠 Algorithm Explanation

We need to count the number of ways a robot can move from the top-left corner to the bottom-right corner in a grid, moving only **right** or **down**, while avoiding obstacles. Obstacles are represented as `1`, free cells as `0`.

We use **Dynamic Programming (DP)**:

- Let `dp[j]` represent the number of ways to reach cell `(i, j)` in the current row.
- Transition:
  - If the current cell is an obstacle → `dp[j] = 0`
  - Else → `dp[j] = dp[j] + dp[j-1]` (from top + from left)
- Initialize `dp[0] = 1` (if the start is free).
- The result is `dp[n-1]` after processing all rows.

This reduces space from `O(m*n)` to `O(n)`.

---

### 🪜 Steps

1. **Check Base Case**  
   - If start `(0,0)` or end `(m-1,n-1)` is blocked → return `0`.

2. **Initialize DP Array**  
   - Use a 1D array `dp` of size `n`, set `dp[0] = 1`.

3. **Iterate Over Grid**  
   - For each cell `(i, j)`:
     - If obstacle → `dp[j] = 0`
     - Else if `j > 0` → `dp[j] += dp[j-1]`

4. **Return Result**  
   - Final answer is `dp[n-1]`.

---

## ✅ Constraints

- `1 <= m, n <= 100`
- `grid[i][j]` is `0` (free space) or `1` (obstacle).
- Answer ≤ `2 * 10^9`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(m·n)** |
| Space Complexity  | **O(n)**   |

---
