# LeetCode Problem: Range Sum Query - Mutable

- **Problem Link**: [Range Sum Query - Mutable – LeetCode](https://leetcode.com/problems/range-sum-query-mutable/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/range-sum-query-mutable/solutions/)

---

## Algorithm

Design `NumArray` supporting point updates and range sums in better than **O(n)**
per operation. Unlike [Q303](../Q303%20Range%20Sum%20Query%20Immutable/), the
array changes, so a static prefix table is not enough.

### Fenwick tree (Binary Indexed Tree)

Use a 1-indexed `tree` where `prefix(i)` returns the sum of the first `i`
elements in **O(log n)**:

- **Build** — add each `nums[i]` at index `i + 1`.
- **update(index, val)** — apply delta `val - nums[index]` at index `index + 1`.
- **sumRange(left, right)** — `prefix(right + 1) - prefix(left)`.

Each `add` walks indices `i += i & -i`; each `prefix` walks `i -= i & -i`.

Example: `nums = [1, 3, 5]` → `sumRange(0, 2) = 9`; after `update(1, 2)` →
`sumRange(0, 2) = 8`.

---

## Constraints

- `1 <= nums.length <= 3 * 10^4`
- `-100 <= nums[i] <= 100`
- `0 <= index < nums.length`
- `-100 <= val <= 100`
- `0 <= left <= right < nums.length`
- At most `3 * 10^4` calls to `update` and `sumRange`

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n log n)** build; **O(log n)** per `update` and `sumRange`                  |
| Space Complexity   | **O(n)** — Fenwick tree and a copy of nums for delta updates                    |

---
<!-- markdownlint-enable MD013 -->
