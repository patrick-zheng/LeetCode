# LeetCode Problem: Majority Element II

- **Problem Link**: [Majority Element II - LeetCode](https://leetcode.com/problems/majority-element-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/majority-element-ii/solutions/)

---

## Algorithm

Any value that appears more than `n / 3` times is a valid answer.

There can be at most two such values, because three different values each
appearing more than `n / 3` times would exceed `n`.

So we use the Boyer-Moore voting idea with two candidates:

1. Maintain `(candidate1, count1)` and `(candidate2, count2)`.
2. For each number:
   - if it matches a candidate, increment that candidate's count
   - else if one count is zero, replace that candidate with this number
   - else decrement both counts
3. After one pass, candidates are only potential answers.
4. Run a second pass to count the real frequencies of the two candidates.
5. Keep candidates with frequency `> floor(n / 3)`.

This gives linear time with constant extra space.

---

## Constraints

- `1 <= nums.length <= 5 * 10^4`
- `-10^9 <= nums[i] <= 10^9`

---

## Complexity

<!-- markdownlint-disable MD013 -->
| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** - one pass for candidate selection and one pass for verification       |
| Space Complexity   | **O(1)** auxiliary space (output list excluded by convention)                   |
<!-- markdownlint-enable MD013 -->

---
