# LeetCode Problem: Contains Duplicate

- **Problem Link**: [Contains Duplicate – LeetCode](https://leetcode.com/problems/contains-duplicate/)

---

## Algorithm

We need to know whether any integer appears more than once. A **hash set** stores values we have already seen while scanning the array once.

- For each `x`, if `x` is already in the set, return **true** (duplicate found).
- Otherwise insert `x` and continue.
- If the scan finishes, return **false**.

Sorting would be **O(n log n)** time and is not better here.

---

## Constraints

- `1 <= nums.length <= 10^5`
- `-10^9 <= nums[i] <= 10^9`

---

## Complexity

| Metric           | Complexity |
|------------------|------------|
| Time Complexity  | O(n)       |
| Space Complexity | O(n)       |

---
