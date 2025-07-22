# 🧩 LeetCode Problem: Sudoku Solver

- **Problem Link**: [Sudoku Solver – LeetCode](https://leetcode.com/problems/sudoku-solver/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/sudoku-solver/solutions/)

---

## 🧠 Algorithm Explanation

This problem is best solved using **Backtracking with Constraint Propagation**. The idea is to fill each empty cell with a valid number (1–9) that does not violate Sudoku constraints:

- Each number must appear exactly once in each row.
- Each number must appear exactly once in each column.
- Each number must appear exactly once in each 3x3 subgrid.

By maintaining **sets of available digits** for rows, columns, and boxes, we efficiently narrow down possible candidates for each cell. This dramatically reduces unnecessary recursive calls and speeds up the solving process.

---

### 🪜 Steps

1. **Initialize Available Sets**  
   Track which digits are still allowed in each row, column, and 3x3 box using `set`.

2. **Preprocess Board**  
   Iterate through the initial board and remove used digits from their respective sets.

3. **Collect Empty Cells**  
   Store coordinates of all empty cells for ordered processing.

4. **Backtrack**  
   Recursively attempt to fill each empty cell with a valid digit from the intersection of allowed digits in that row, column, and box.

5. **Prune Invalid Paths**  
   If placing a digit leads to an invalid state later, undo the move and try the next candidate (backtracking).

---

## ✅ Constraints

- `board.length == 9`
- `board[i].length == 9`
- `board[i][j]` is a digit `'1'`–`'9'` or `'.'`.
- There is **exactly one solution** to the puzzle.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity                                |
|-------------------|--------------------------------------------|
| Time Complexity   | O(9^m) where `m` is the number of empty cells |
| Space Complexity  | O(1) – constant space for fixed-size sets |

---
