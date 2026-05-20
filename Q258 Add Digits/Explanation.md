# LeetCode Problem: Add Digits

- **Problem Link**: [Add Digits – LeetCode](https://leetcode.com/problems/add-digits/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/add-digits/solutions/)

---

## Algorithm

Repeatedly sum decimal digits until one digit remains. That value is the
**digital root** of `num` (with `0` mapping to `0`).

### Digital root — **O(1)** time, **O(1)** space

For `num > 0`, the answer is `1 + (num - 1) % 9`. Each digit-sum step preserves
`num mod 9` except when the running total is `9` (then the next sum is `9`, not
`0`), which the `1 + (num - 1) % 9` formula encodes. Handle `num == 0` separately.

A loop that sums digits each round is **O(log n)** time with **O(1)** extra space
and is also correct, but the closed form is optimal here.

---

## Constraints

- `0 <= num <= 10^9`

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(1)** — closed-form digital root; no digit loop.                             |
| Space Complexity   | **O(1)** — constant extra memory.                                               |

---
<!-- markdownlint-enable MD013 -->
