# 🧩 LeetCode Problem: Spiral Matrix

- **Problem Link**: [54. Spiral Matrix – LeetCode](https://leetcode.com/problems/spiral-matrix/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/spiral-matrix/solutions/)

---

## 🧠 Algorithm Explanation

We maintain four pointers (`top`, `bottom`, `left`, `right`) representing the current boundaries of the unvisited part of the matrix.  
We traverse in four directions — left → right, top → bottom, right → left, and bottom → top — shrinking the boundaries after each traversal until all elements are visited.  
This approach is efficient because it visits each element exactly once.

---

### 🪜 Steps

1. **Initialize Boundaries**:  
   - `top = 0`, `bottom = m - 1`, `left = 0`, `right = n - 1`.

2. **Traverse in Spiral Order**:  
   - Go **left → right** along `top` row, then increment `top`.  
   - Go **top → bottom** along `right` column, then decrement `right`.  
   - If `top <= bottom`, go **right → left** along `bottom` row, then decrement `bottom`.  
   - If `left <= right`, go **bottom → top** along `left` column, then increment `left`.

3. **Repeat** until `top > bottom` or `left > right`.

---

## ✅ Constraints

- `m == len(matrix)`
- `n == len(matrix[i])`
- `1 <= m, n <= 10`
- `-100 <= matrix[i][j] <= 100`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(m × n)   |
| Space Complexity  | O(1)       |

---
