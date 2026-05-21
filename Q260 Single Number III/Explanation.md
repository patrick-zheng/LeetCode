# LeetCode Problem: Single Number III

- **Problem Link**: [Single Number III – LeetCode](https://leetcode.com/problems/single-number-iii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/single-number-iii/solutions/)

---

## Algorithm

Every value appears **twice** except **two** distinct singles `x` and `y`. XOR all
numbers: pairs cancel (`a ^ a = 0`), leaving `xor_all = x ^ y`.

Because `x != y`, `xor_all` is non-zero, so some bit differs between `x` and `y`.
Let `bit` be any set bit of `xor_all` (we use `xor_all & -xor_all`, the lowest set
bit).

Split the array by whether that bit is set in each number, and XOR each group.
Numbers that appear twice land in the **same** group and cancel; the two singles
sit in **different** groups, so each group XORs to one answer.

Return the two results in any order.

---

## Constraints

- `2 <= nums.length <= 3 * 10^4`
- `-2^31 <= nums[i] <= 2^31 - 1`
- Each integer appears **twice** except for two integers that appear **once**

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** — three linear passes: XOR all, split bit, XOR groups.                 |
| Space Complexity   | **O(1)** — only a few integer accumulators besides the input.                   |

---
<!-- markdownlint-enable MD013 -->
