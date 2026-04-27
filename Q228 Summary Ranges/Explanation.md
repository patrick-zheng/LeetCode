# LeetCode Problem: Summary Ranges

- **Problem Link**: [Summary Ranges - LeetCode](https://leetcode.com/problems/summary-ranges/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/summary-ranges/solutions/)

---

## Algorithm

The input is already sorted and has no duplicates, so each range is a maximal run
of consecutive integers.

Scan once with two pointers:

- `start`: first value of the current range
- `prev`: most recent value in the current range

For each next number:

1. If it is consecutive (`num == prev + 1`), extend the current range.
2. Otherwise, close the current range:
   - if `start == prev`, append `"start"`
   - else append `"start->prev"`
3. Start a new range at this number.

After the loop, close the final range using the same rule.

This greedily emits every maximal consecutive segment exactly once.

---

## Constraints

- `0 <= nums.length <= 20`
- `-2^31 <= nums[i] <= 2^31 - 1`
- All values of `nums` are unique
- `nums` is sorted in ascending order

---

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** - single pass through `nums`                                           |
| Space Complexity   | **O(1)** auxiliary; output array excluded by convention                         |

---
