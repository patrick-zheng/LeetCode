# 🧩 LeetCode Problem: Valid Sudoku

- **Problem Link**: [Valid Sudoku – LeetCode](https://leetcode.com/problems/valid-sudoku/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/valid-sudoku/solutions/)

---

## 🧠 Algorithm Explanation

The goal is to validate whether a given 9x9 Sudoku board is valid. A valid Sudoku board must satisfy the following rules:

- Each row must contain the digits 1-9 without repetition.
- Each column must contain the digits 1-9 without repetition.
- Each of the 9 sub-boxes (3x3 regions) must contain the digits 1-9 without repetition.

To efficiently check for these constraints, we use:

- A list of sets to track seen numbers for each row.
- A list of sets to track seen numbers for each column.
- A list of sets to track seen numbers in each 3x3 box.

This approach is used because:

- Set lookups (`in`) are **O(1)** on average.
- We avoid re-scanning the board multiple times.

---

### 🪜 Steps

1. **Initialize tracking structures**:
   - `rows`, `cols`, and `boxes` as lists of 9 sets each.

2. **Iterate through each cell** in the 9x9 grid:
   - Skip empty cells (represented by `'.'`).
   - Compute the 3x3 box index using the formula: `(row // 3) * 3 + (col // 3)`.

3. **Check for duplicates**:
   - If the current number is already present in its corresponding row, column, or box, return `False`.

4. **Update tracking sets**:
   - Add the number to the appropriate row, column, and box sets.

5. **Return True** after iterating all cells if no conflicts are found.

---

## ✅ Constraints

- The board is always a 9 x 9 grid.
- Each cell contains `'1'` to `'9'` or `'.'`.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(1)       |
| Space Complexity  | O(1)       |

> Even though we iterate through 81 cells, this is treated as constant time due to the fixed board size.
