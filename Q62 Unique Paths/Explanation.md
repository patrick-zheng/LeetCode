# 🧩 LeetCode Problem: Unique Paths

- **Problem Link**: [Unique Paths – LeetCode](https://leetcode.com/problems/unique-paths/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/unique-paths/solutions/)

---

## 🧠 Algorithm Explanation

The problem asks for the number of unique paths a robot can take from the top-left corner to the bottom-right corner of an `m x n` grid, moving only down or right.

This is a **combinatorial problem**:

- The robot must make `(m-1)` downward moves and `(n-1)` rightward moves, in any order.
- The total number of moves is `(m-1) + (n-1) = m + n - 2`.
- The number of unique ways to arrange these moves is given by the **binomial coefficient**:
  
\[
\binom{m+n-2}{m-1} = \frac{(m+n-2)!}{(m-1)!(n-1)!}
\]

We use this formula because it computes the result in **O(min(m, n))**, far more efficient than dynamic programming approaches (`O(m*n)`).

---

### 🪜 Steps

1. **Determine total moves**: `total = m + n - 2`.
2. **Choose the smaller of `(m-1)` or `(n-1)`** as `k` to minimize computation.
3. **Compute the binomial coefficient iteratively**:
   - Multiply by each term in the numerator.
   - Divide by each term in the denominator (division is always exact for binomial coefficients).

---

## ✅ Constraints

- `1 <= m, n <= 100`
- The answer is guaranteed to be ≤ `2 * 10^9`.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity       |
|-------------------|------------------|
| Time Complexity   | O(min(m, n))     |
| Space Complexity  | O(1)            |

---
