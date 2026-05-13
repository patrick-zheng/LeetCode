# LeetCode Problem: Search a 2D Matrix II

- **Problem Link**: [Search a 2D Matrix II – LeetCode](https://leetcode.com/problems/search-a-2d-matrix-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/search-a-2d-matrix-ii/solutions/)

---

## Algorithm

Rows increase left to right; columns increase top to bottom. Treat the grid as a
**partial order**: everything strictly **left** of position \((r, c)\) is
smaller, everything strictly **below** is larger.

Start at the **top-right** corner \((0, n-1)\) (or symmetrically bottom-left).
Compare `matrix[r, c]` with `target`:

- **Equal** → found.
- **Greater than target** → no larger values in that column above the current
  row, so the target cannot lie in column `c`; move **left** (`c -= 1`).
- **Less than target** → no smaller values in that row to the right, so the
  target cannot lie in row `r`; move **down** (`r += 1`).

Each step discards a full row segment or column segment, so at most **\(m + n\)**
steps. The walk either finds `target` or exits the grid.

---

## Constraints

- `m == matrix.length`
- `n == matrix[i].length`
- `1 <= m, n <= 300`
- `-10^9 <= matrix[i, j], target <= 10^9`

---

## Complexity

<!-- markdownlint-disable MD013 -->

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(m + n)** — each step drops a full row or column along the search path       |
| Space Complexity   | **O(1)** — constant extra space for indices only                                |

<!-- markdownlint-enable MD013 -->

---
