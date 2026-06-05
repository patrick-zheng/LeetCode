# LeetCode Problem: Move Zeroes

- **Problem Link**: [Move Zeroes – LeetCode](https://leetcode.com/problems/move-zeroes/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/move-zeroes/solutions/)

---

## Algorithm

Move all **0**s to the end of `nums` **in-place**, keeping the relative order
of non-zero elements.

Use a **write pointer** `write` for the next position of a non-zero value.
Scan `i` from left to right:

1. If `nums[i] != 0`, swap it into `nums[write]` (skip swap when `i == write`),
   then increment `write`.
2. If `nums[i] == 0`, leave it; it will be overwritten or swapped away later.

Non-zeros stay in order at the front; remaining slots end up as zero after
the scan. One pass, constant extra space.

Example: `[0,1,0,3,12]` → `[1,3,12,0,0]`.

---

## Constraints

- `1 <= nums.length <= 10^4`
- `-2^31 <= nums[i] <= 2^31 - 1`

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** — single pass over `nums`                                              |
| Space Complexity   | **O(1)** — only pointer indices; in-place swaps                                 |

---
<!-- markdownlint-enable MD013 -->
