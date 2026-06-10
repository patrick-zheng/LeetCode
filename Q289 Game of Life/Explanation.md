# LeetCode Problem: Game of Life

- **Problem Link**: [Game of Life – LeetCode](https://leetcode.com/problems/game-of-life/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/game-of-life/solutions/)

---

## Algorithm

Simulate one generation of Conway's Game of Life on an `m x n` board (`0` dead,
`1` live). Update **in-place** with **O(1)** extra space.

### Rules

- Live cell with fewer than 2 or more than 3 live neighbors dies.
- Dead cell with exactly 3 live neighbors becomes live.
- Otherwise the cell stays the same.

### Intermediate states

A second board would use **O(mn)** space. Instead mark transitions on the same
grid:

- `2` — was live (`1`), becomes dead (`0`)
- `3` — was dead (`0`), becomes live (`1`)

When counting neighbors, treat `1` and `2` as live (still alive for this step).

1. Scan every cell, count live neighbors in the 8 surrounding cells, set `2`
   or `3` where the next state differs.
2. Second pass: convert `2` to `0` and `3` to `1`.

---

## Constraints

- `m == board.length`
- `n == board[i].length`
- `1 <= m, n <= 25`
- `board[i][j]` is `0` or `1`

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(mn)** — each cell checks up to 8 neighbors twice                            |
| Space Complexity   | **O(1)** — only temporary counters; board updated in place                      |

---
<!-- markdownlint-enable MD013 -->
