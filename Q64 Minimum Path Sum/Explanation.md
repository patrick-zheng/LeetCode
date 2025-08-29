# 🧩 LeetCode Problem: Minimum Path Sum

- **Problem Link**: [Minimum Path Sum – LeetCode](https://leetcode.com/problems/minimum-path-sum/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/minimum-path-sum/solutions/)

---

## 🧠 Algorithm Explanation

We use dynamic programming to compute the minimum path sum.  
Each cell’s value is updated as the current grid value plus the minimum of the top or left neighbor.  
This ensures at every step we choose the optimal subpath.  
We can optimize space by only keeping one row at a time.

---

### 🪜 Steps

1. **Initialize** the first row and column since they can only be reached from one direction.  
2. **Iterate** through the grid: for each cell, compute `grid[i][j] += min(grid[i-1][j], grid[i][j-1])`.  
3. **Result** is stored in the bottom-right cell.

---

## ✅ Constraints

- `1 ≤ m, n ≤ 200`  
- `0 ≤ grid[i][j] ≤ 100`  
- Only right and down moves are allowed.  

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(m·n)     |
| Space Complexity  | O(n)       |

---
