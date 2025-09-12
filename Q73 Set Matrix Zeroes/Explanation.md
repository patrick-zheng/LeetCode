# 🧩 LeetCode Problem: Set Matrix Zeroes

- **Problem Link**: [Set Matrix Zeroes – LeetCode](https://leetcode.com/problems/set-matrix-zeroes/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/set-matrix-zeroes/solutions/)

---

## 🧠 Algorithm Explanation

The problem asks us to set an entire row and column to zero if any cell in a given matrix is zero, and we must do this **in-place** without using extra space for another matrix.

The naive solution would be to use additional arrays to keep track of rows and columns that should be zeroed, but that requires **O(m + n)** extra space.  

The optimized solution uses the **first row and first column of the matrix as markers** to remember which rows and columns should be zeroed. To avoid overwriting useful information, we separately track whether the first row or first column themselves need to be zeroed.

This approach ensures we only use **O(1) extra space** beyond the input matrix.

---

### 🪜 Steps

1. **Check if first row and first column need to be zeroed**  
   - Iterate through the first row and first column to see if they contain any zeros.  
   - Store results in `first_row_has_zero` and `first_col_has_zero`.

2. **Mark rows and columns using first row and column**  
   - Traverse the rest of the matrix (excluding the first row and column).  
   - If a cell `(i, j)` is zero, mark its row and column by setting `matrix[i][0] = 0` and `matrix[0][j] = 0`.

3. **Apply markers to update the matrix**  
   - Iterate again through the matrix (excluding first row/column).  
   - If either its row marker or column marker is zero, set the cell to zero.

4. **Update the first row and column if needed**  
   - If `first_row_has_zero` is true, zero out the first row.  
   - If `first_col_has_zero` is true, zero out the first column.

---

## ✅ Constraints

- `m == matrix.length`
- `n == matrix[0].length`
- `1 <= m, n <= 200`
- `-2^31 <= matrix[i][j] <= 2^31 - 1`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(m × n)** — We scan the entire matrix twice. |
| Space Complexity  | **O(1)** — We only use variables to track the first row/column. |

---
