# 🧩 LeetCode Problem: Rotate Image

- **Problem Link**: [Rotate Image – LeetCode](https://leetcode.com/problems/rotate-image/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/rotate-image/solutions/)

---

## 🧠 Algorithm Explanation

The problem requires rotating an **n × n** 2D matrix 90 degrees clockwise **in place**, meaning no extra matrix can be used.  
The most efficient approach is to **transpose the matrix** and then **reverse each row**.  

- **Transpose** changes rows into columns by swapping `matrix[i][j]` with `matrix[j][i]` for `i < j`.  
- **Row reversal** shifts elements to their correct position for a 90° rotation.

This method is chosen because:

- It uses **O(1)** extra space.
- It moves each element exactly once, achieving **O(n²)** time complexity.
- It’s simpler and faster than rotating layer by layer for Python.

---

### 🪜 Steps

1. **Step 1: Transpose the matrix**  
   Swap each element at `(i, j)` with `(j, i)` for all `i < j`.

2. **Step 2: Reverse each row**  
   Reverse each row in place to complete the 90° rotation.

3. **Step 3: Done**  
   The matrix is now rotated 90 degrees clockwise.

---

## ✅ Constraints

- `n == matrix.length == matrix[i].length`
- `1 <= n <= 20`
- `-1000 <= matrix[i][j] <= 1000`
- Rotation must be done **in place** without allocating another matrix.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n²)      |
| Space Complexity  | O(1)       |
