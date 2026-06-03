# LeetCode Problem: Perfect Squares

- **Problem Link**: [Perfect Squares – LeetCode](https://leetcode.com/problems/perfect-squares/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/perfect-squares/solutions/)

---

## Algorithm

Find the **minimum** number of perfect squares that sum to `n`. Treat this as
**shortest path**: start at `n`, and in one step subtract any perfect square
`sq` to reach `n - sq`. The answer is the fewest steps to reach `0`.

### Breadth-first search

1. Precompute all squares `1, 4, 9, ...` up to `n`.
2. BFS from `n` with a `visited` set so each remainder is enqueued once.
3. Each BFS level is one square used. When some `value - sq == 0`, return the
   current level count.
4. Enqueue positive unvisited remainders `value - sq`.

BFS finds the minimum number of squares because edges have equal weight (one
square per step) and we expand in order of increasing depth.

Example: `n = 13` → `13 → 9 → 4 → 0` in two steps (`13 = 9 + 4`).

---

## Constraints

- `1 <= n <= 10^4`

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n sqrt n)** — each state tried once, up to `sqrt(n)` squares per state      |
| Space Complexity   | **O(n)** — queue and visited set hold up to `O(n)` remainders                   |

---
<!-- markdownlint-enable MD013 -->
