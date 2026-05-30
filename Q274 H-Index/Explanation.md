# LeetCode Problem: H-Index

- **Problem Link**: [H-Index – LeetCode](https://leetcode.com/problems/h-index/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/h-index/solutions/)

---

## Algorithm

The **h-index** is the largest `h` such that at least `h` papers have
**at least** `h` citations each.

Sorting descending and scanning is **O(n log n)**. A **counting bucket**
is **O(n)**:

1. Let `n = len(citations)`. Build `bucket` of size `n + 1`.
2. For each citation `c`: increment `bucket[min(c, n)]` (values `>= n`
   all behave like `n` for h-index purposes).
3. Walk `h` from `n` down to `0`, accumulating how many papers have at
   least `h` citations. The first `h` with `papers >= h` is the answer.

If no positive `h` qualifies, return **0**.

---

## Constraints

- `n == citations.length`
- `1 <= n <= 10^5`
- `0 <= citations[i] <= 10^3`

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** — one pass to fill buckets, one pass over `n + 1` slots                |
| Space Complexity   | **O(n)** — bucket array of size `n + 1`                                         |

---
<!-- markdownlint-enable MD013 -->
