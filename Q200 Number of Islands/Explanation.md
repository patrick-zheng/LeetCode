# 🧩 LeetCode Problem: Number of Islands

- **Problem Link**: [Number of Islands – LeetCode](https://leetcode.com/problems/number-of-islands/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/number-of-islands/solutions/)

---

## 🧠 Algorithm Explanation

We traverse every cell in the grid. Whenever we find a land cell (`'1'`) that has not been visited yet, we have discovered a new island. We then use Depth-First Search (DFS) to explore and mark all horizontally and vertically connected land cells as visited so they are not counted again. This algorithm is used because each cell is processed at most once, making it optimal for this problem.

---

### 🪜 Steps

1. **Step 1**: Loop through every cell in the grid.

2. **Step 2**: When a cell contains `'1'`, increment the island count and start a DFS from that cell.

3. **SStep 3**: In the DFS, mark the current land cell as visited and continue exploring its up, down, left, and right neighbors.

---

## ✅ Constraints

- `m == grid.length`
- `n == grid[i].length`
- `1 <= m, n <= 300`
- `grid[i][j]` is `'0'` or `'1'`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(m × n)   |
| Space Complexity  | O(m × n)   |

---
