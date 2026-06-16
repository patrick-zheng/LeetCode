# LeetCode Problem: Find Median from Data Stream

- **Problem Link**: [Find Median from Data Stream – LeetCode](https://leetcode.com/problems/find-median-from-data-stream/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/find-median-from-data-stream/solutions/)

---

## Algorithm

Support streaming `addNum` and `findMedian` on integers. Sorting after every
insert is too slow.

### Two heaps

Split values into a **lower half** and **upper half**:

- `low` — max-heap of the smaller values
- `high` — min-heap of the larger values

Invariant: every value in `low` is `<=` every value in `high`, and
`len(low)` is `len(high)` or `len(high) + 1`.

**`addNum(num)`**

1. Push `num` into `low`.
2. Move `low`'s maximum to `high`.
3. If `low` is smaller than `high`, move `high`'s minimum back to `low`.

**`findMedian()`**

- If `low` has one extra element, median is `low`'s top.
- Otherwise median is the average of `low`'s top and `high`'s top.

Each insert is **O(log n)**; median read is **O(1)**.

---

## Constraints

- `-10^5 <= num <= 10^5`
- At most `5 * 10^4` calls to `addNum` and `findMedian`

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(log n)** per `addNum`; **O(1)** per `findMedian`                            |
| Space Complexity   | **O(n)** — all numbers stored across the two heaps                              |

---
<!-- markdownlint-enable MD013 -->
