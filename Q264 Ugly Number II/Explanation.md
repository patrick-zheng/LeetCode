# LeetCode Problem: Ugly Number II

- **Problem Link**: [Ugly Number II – LeetCode](https://leetcode.com/problems/ugly-number-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/ugly-number-ii/solutions/)

---

## Algorithm

The first ugly number is **1**. Every later ugly number equals a **smaller**
ugly number times **2**, **3**, or **5**. Checking every integer with
`isUgly` is too slow.

### Three-pointer merge (dynamic sequence)

Build `ugly[0..n-1]` in sorted order:

1. Set `ugly[0] = 1`. Maintain indices `i2`, `i3`, `i5` pointing at ugly
   values that will generate the next candidates `ugly[i2]*2`, `ugly[i3]*3`,
   `ugly[i5]*5`.
2. For `i = 1 .. n-1`, set `ugly[i]` to the **minimum** of those three
   candidates.
3. Advance each pointer whose candidate **equals** `ugly[i]` (use separate
   `if`s so duplicates like `6 = 2*3 = 3*2` advance more than one pointer).
4. Return `ugly[n-1]`.

This is the same idea as merging three sorted streams
`2, 4, 8, ...`, `3, 6, 9, ...`, and `5, 10, 15, ...` without storing them
all.

---

## Constraints

- `1 <= n <= 1690`
- The answer fits in a 32-bit signed integer for this bound

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** — one iteration per ugly number; **O(1)** work per step                |
| Space Complexity   | **O(n)** — array `ugly` of length `n`                                           |

---
<!-- markdownlint-enable MD013 -->
