# LeetCode Problem: Contains Duplicate II

- **Problem Link**: [Contains Duplicate II – LeetCode](https://leetcode.com/problems/contains-duplicate-ii/)

---

## Algorithm

We need two **distinct indices** `i` and `j` with `nums[i] == nums[j]` and `|i - j| <= k`. Equivalently, for each index `i`, we only care whether `nums[i]` appeared among indices `i - k` through `i - 1`.

Maintain a **hash set** of values in the current **sliding window** of length at most `k + 1` (indices `i - k` … `i` after including the current element).

1. Scan `i` from `0` to `n - 1`.
2. If `nums[i]` is already in the set, a duplicate exists within distance `k` → return **true**.
3. Insert `nums[i]`.
4. If `i >= k`, remove `nums[i - k]` so the window never spans more than `k` steps backward from `i`.

An alternative is a hash map from value to **last index**; each step updates in **O(1)** as well. The set approach matches the window size directly.

---

## Constraints

- `1 <= nums.length <= 10^5`
- `-10^9 <= nums[i] <= 10^9`
- `0 <= k <= 10^5`

---

## Complexity

| Metric             | Complexity                                           |
|--------------------|------------------------------------------------------|
| Time Complexity    | O(n) — single pass, set/map O(1) average             |
| Space Complexity   | O(min(n, k + 1)) — window size is bounded by `k + 1` |

---
