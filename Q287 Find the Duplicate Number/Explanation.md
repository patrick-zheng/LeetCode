# LeetCode Problem: Find the Duplicate Number

- **Problem Link**: [Find the Duplicate Number – LeetCode](https://leetcode.com/problems/find-the-duplicate-number/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/find-the-duplicate-number/solutions/)

---

## Algorithm

`nums` has length `n + 1` with values in `1..n`, and exactly one value repeats.
The array must not be modified and extra space must be **O(1)**.

### Floyd's cycle detection (linked list view)

Treat each index `i` as a node with an edge to `nums[i]`. Because a value
appears twice, two indices point to the same next node, so the walk must
enter a cycle. The duplicate value is the cycle entry (same idea as
[Linked List Cycle II](https://leetcode.com/problems/linked-list-cycle-ii/)).

1. **Find a meeting point**: start `slow` at `nums[0]` and `fast` at
   `nums[nums[0]]`; advance `slow` one step and `fast` two steps until they
   meet inside the cycle.
2. **Find cycle entrance**: reset `slow` to index `0`, move `slow` and `fast`
   one step at a time until they meet again. That index is the duplicate.

No sorting, no hash set, no array edits.

---

## Constraints

- `1 <= n <= 10^5`
- `nums.length == n + 1`
- `1 <= nums[i] <= n`
- Exactly one integer appears two or more times

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** — pointer walk is linear in array length                               |
| Space Complexity   | **O(1)** — only two index pointers; array unchanged                             |

---
<!-- markdownlint-enable MD013 -->
