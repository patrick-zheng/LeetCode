# 🧩 LeetCode Problem: Search a 2D Matrix

- **Problem Link**: [Search a 2D Matrix – LeetCode](https://leetcode.com/problems/search-a-2d-matrix/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/search-a-2d-matrix/solutions/)

---

## 🧠 Algorithm Explanation

We treat the `m x n` matrix as a flattened **sorted 1D array**.  
Because:

1. Each row is sorted in ascending order.  
2. The first element of each row is greater than the last element of the previous row.  

This means the entire matrix is sorted when viewed as a single array of size `m * n`.  
Thus, we can apply **binary search** directly over the virtual 1D array, then map the mid index back to `(row, col)` using:

- `row = mid // n`
- `col = mid % n`

This guarantees `O(log(m * n))` time complexity.

---

### 🪜 Steps

1. **Flatten indexing**  
   Treat the matrix as one array of length `m * n`.  

2. **Binary search**  
   Apply standard binary search between indices `[0, m*n - 1]`.  

3. **Map index to 2D**  
   For mid index, compute:  
   - `row = mid // n`  
   - `col = mid % n`  
   Compare `matrix[row][col]` with `target`.  

4. **Return result**  
   If found, return `true`; otherwise, continue binary search until the search interval is empty.  

---

## ✅ Constraints

- `m == matrix.length`  
- `n == matrix[i].length`  
- `1 <= m, n <= 100`  
- `-10⁴ <= matrix[i][j], target <= 10⁴`  

---

## ⏱ Time and Space Complexity

| Metric            | Complexity     |
|-------------------|----------------|
| Time Complexity   | `O(log(m * n))` |
| Space Complexity  | `O(1)`         |

---
