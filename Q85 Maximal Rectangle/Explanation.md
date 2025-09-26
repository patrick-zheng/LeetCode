# 🧩 LeetCode Problem: Maximal Rectangle

- **Problem Link**: [Maximal Rectangle – LeetCode](https://leetcode.com/problems/maximal-rectangle/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/maximal-rectangle/solutions/)

---

## 🧠 Algorithm Explanation

The problem asks us to find the largest rectangle containing only `1`s in a binary matrix.  
We can solve this by treating each row of the matrix as the base of a histogram:

- For each column, compute the "height" of consecutive `1`s up to the current row.
- Then, for that histogram, compute the **largest rectangle in a histogram** using a monotonic stack.
- Repeat for every row, updating the global maximum rectangle area.

This works because every rectangle in the matrix must end at some row, so by checking all rows as bases we ensure completeness.

---

### 🪜 Steps

1. **Build histogram heights**:  
   For each row, update a `heights` array where `heights[c]` represents the consecutive count of `1`s ending at that row in column `c`.

2. **Apply largest rectangle in histogram algorithm**:  
   Use a monotonic increasing stack to compute the maximum rectangle area for the histogram represented by `heights`.

3. **Update maximum**:  
   After processing each row, keep track of the maximum rectangle area found so far.

---

## ✅ Constraints

- `rows == matrix.length`
- `cols == matrix[i].length`
- `1 <= rows, cols <= 200`
- `matrix[i][j]` is either `'0'` or `'1'`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(rows × cols) |
| Space Complexity  | O(cols) |

---
