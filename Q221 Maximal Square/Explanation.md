# LeetCode Problem: Maximal Square

- **Problem Link**: [Maximal Square – LeetCode](https://leetcode.com/problems/maximal-square/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/maximal-square/solutions/)

---

## Algorithm

We want the **largest square of `'1'` cells** and must return its **area** (side length squared).

### Dynamic programming — **O(m·n)** time, **O(n)** space

Let `dp[i, j]` be the **side length** of the largest all-`'1'` square whose **bottom-right corner** is `(i, j)`.

- If `matrix[i, j] == '0'`, then `dp[i, j] = 0`.
- If it is `'1'`, the square must extend the square from **up**, **left**, and **up-left** by one cell:

\[
dp_{i,j} = \min(dp_{i-1,j},\, dp_{i,j-1},\, dp_{i-1,j-1}) + 1
\]

The **minimum** of the three neighbors is the limiting side length we can extend; adding `1` accounts for the new bottom-right cell.

The answer is \(\max_{i,j} dp_{i,j}^2\).

**Space optimization:** only the **previous row** (or one array updated left-to-right) is needed. When scanning row `i` left to right, keep `northwest = dp[i-1, j-1]` in a scalar: before overwriting `dp[j]` (which holds `dp[i-1, j]`), save it to update `northwest` for the next column.

---

## Constraints

- `m == matrix.length`
- `n == matrix[i].length`
- `1 <= m, n <= 300`
- `matrix[i, j]` is `'0'` or `'1'`

---

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(m·n)** — one pass per cell                                                  |
| Space Complexity   | **O(n)** — one DP row (`O(m·n)` if storing full `dp` table)                     |

---
