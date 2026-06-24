# LeetCode Problem: Range Sum Query 2D - Immutable

- **Problem Link**: [Range Sum Query 2D - Immutable – LeetCode](https://leetcode.com/problems/range-sum-query-2d-immutable/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/range-sum-query-2d-immutable/solutions/)

---

## Algorithm

Design `NumMatrix` so `sumRegion(row1, col1, row2, col2)` returns the sum of
the inclusive rectangle in **O(1)** time after preprocessing. The matrix never
changes.

### 2D prefix sums

Pad the matrix with an extra top row and left column of zeros. Let
$P_{i,j}$ be the sum of `matrix[0..i-1][0..j-1]` (using 1-based indices on
the padded grid):

$$
P_{i,j} = P_{i-1,j} + P_{i,j-1} - P_{i-1,j-1} + \text{matrix}[i-1][j-1]
$$

For a query rectangle from `(row1, col1)` to `(row2, col2)`:

$$
\begin{aligned}
\text{sumRegion} = &\, P_{row2+1,\, col2+1} - P_{row1,\, col2+1} \\
& - P_{row2+1,\, col1} + P_{row1,\, col1}
\end{aligned}
$$

This is inclusion–exclusion: take the big prefix sum, subtract the strips above
and to the left, then add back the corner double-subtracted once.

Example: `sumRegion(2, 1, 4, 3)` on the LeetCode sample matrix returns `8`.

---

## Constraints

- `m == matrix.length`
- `n == matrix[i].length`
- `1 <= m, n <= 200`
- `-10^5 <= matrix[i][j] <= 10^5`
- `0 <= row1 <= row2 < m`
- `0 <= col1 <= col2 < n`
- At most `10^4` calls to `sumRegion`

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(m·n)** build prefix once; each `sumRegion` is **O(1)**                      |
| Space Complexity   | **O(m·n)** — padded prefix table of size `(m + 1) by (n + 1)`                   |

---
<!-- markdownlint-enable MD013 -->
