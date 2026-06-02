# LeetCode Problem: H-Index II

- **Problem Link**: [H-Index II – LeetCode](https://leetcode.com/problems/h-index-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/h-index-ii/solutions/)

---

## Algorithm

The array is already sorted in non-decreasing order. Let `n = len(citations)`.
If we choose index `i` as the first paper in the high-citation suffix, then
there are exactly `n - i` papers with citations at least `citations[i]`.

For a valid h-index `h`, we need at least `h` papers with `>= h` citations.
At position `i`, the candidate is `h = n - i`, and it is feasible exactly
when `citations[i] >= n - i`.

So we binary-search the **leftmost** index where the inequality holds.

- If `citations[mid] >= n - mid`, this index can produce a valid `h`, and a
  smaller index might produce a larger answer, so move left.
- Otherwise move right.

If such index is `i`, answer is `n - i`; if none exists, answer is `0`.

This satisfies the required logarithmic time.

---

## Constraints

- `n == citations.length`
- `1 <= n <= 10^5`
- `0 <= citations[i] <= 1000`
- `citations` is sorted in ascending order

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(log n)** — binary search on the sorted citations array                      |
| Space Complexity   | **O(1)** — only a few index/counter variables                                   |

---
<!-- markdownlint-enable MD013 -->
