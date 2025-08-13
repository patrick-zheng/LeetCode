# 🧩 LeetCode Problem: N-Queens II

- **Problem Link**: [N-Queens II – LeetCode](https://leetcode.com/problems/n-queens-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/n-queens-ii/solutions/)

---

## 🧠 Algorithm Explanation

The N-Queens II problem asks for the total number of distinct ways to place `n` queens on an `n x n` chessboard so that no two queens attack each other.  
We use **backtracking** because we need to explore all valid placements while pruning invalid states early.  
To efficiently track conflicts, we store:

- **cols**: columns that already have queens.
- **diag1**: diagonals from top-left to bottom-right (`row - col` constant).
- **diag2**: diagonals from top-right to bottom-left (`row + col` constant).

This allows constant-time checks for whether a queen can be placed in a given position.

---

### 🪜 Steps

1. **Initialize sets**: Create three sets `cols`, `diag1`, and `diag2` to track occupied columns and diagonals.
2. **Recursive placement**:
   - For each row, try placing a queen in each column.
   - Skip columns/diagonals already under attack.
   - If valid, mark the position and recurse to the next row.
3. **Count solutions**:
   - When we reach `row == n`, it means all queens are placed, so increment the count.
   - Backtrack by removing the current queen and trying other possibilities.

---

## ✅ Constraints

- `1 <= n <= 9`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n!) — worst case due to backtracking over permutations with pruning |
| Space Complexity  | O(n) — for recursion depth and tracking sets |

---
