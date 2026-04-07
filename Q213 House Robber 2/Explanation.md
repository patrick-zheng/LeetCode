# LeetCode Problem 213: House Robber II

- **Problem Link**: [House Robber II – LeetCode](https://leetcode.com/problems/house-robber-ii/description/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/house-robber-ii/solutions/)

---

## Algorithm Explanation

Houses are arranged in a **circle**: the first and last houses are neighbors. You cannot rob two adjacent houses, including this wrap-around pair.

This is [House Robber (198)](https://leetcode.com/problems/house-robber/) on a line, plus one extra constraint: you cannot take both the first and last house in the same plan.

**Key idea:** Any optimal robbery either skips the **last** house or skips the **first** house (or both). You cannot rob both ends, because they are adjacent on the circle.

- If the optimal plan does **not** rob the last house, it is exactly an optimal solution on the subarray `nums[0..n-1]` (indices `0` through `n-2`), a straight line.
- If the optimal plan **does** rob the last house, it cannot rob the first; then it is an optimal solution on `nums[1..n]` (indices `1` through `n-1`).

So the answer is the maximum of:

1. Best loot on houses `0 .. n-2` (linear robber).
2. Best loot on houses `1 .. n-1` (linear robber).

**Edge case:** If `n == 1`, there is only one house; return its value. The two-range split would produce empty ranges, so handle this explicitly.

Each linear segment is solved with the classic **O(1) space** dynamic programming for House Robber I: at each house, either skip it (keep previous best) or rob it (add its value to the best two steps back).

---

### Steps

1. If `len(nums) == 1`, return `nums[0]`.
2. Compute `a` = maximum money robbing linearly on indices `[0, n-2]` (first house allowed, last forbidden).
3. Compute `b` = maximum money robbing linearly on indices `[1, n-1]` (last house allowed, first forbidden).
4. Return `max(a, b)`.

---

## Constraints

- `1 <= nums.length <= 100`
- `0 <= nums[i] <= 1000`

---

## Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n) — two linear passes over at most `n` elements |
| Space Complexity  | O(1) — only a few scalars for DP on each pass |

---
