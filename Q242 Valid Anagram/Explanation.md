# LeetCode Problem: Valid Anagram

- **Problem Link**: [Valid Anagram – LeetCode](https://leetcode.com/problems/valid-anagram/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/valid-anagram/solutions/)

---

## Algorithm

Two strings are anagrams when every character appears the same number of times in
both.

### Frequency counting — **O(n)** time, **O(1)** space

If `len(s) != len(t)`, return **false** immediately.

Use an array of **26** counters (lowercase `a`–`z` only):

1. Walk `s` and `t` in lockstep with `zip`.
2. Increment the bucket for each character in `s`; decrement for `t`.
3. After the scan, every bucket must be **0**.

Sorting both strings would be **O(n log n)** and uses extra space for sorted copies;
counting is better here.

---

## Constraints

- `1 <= s.length, t.length <= 5 * 10^4`
- `s` and `t` consist of lowercase English letters

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** — one pass over `s` and `t` with **O(1)** work per character.          |
| Space Complexity   | **O(1)** — fixed array of 26 counters (lowercase English letters only).         |

---
<!-- markdownlint-enable MD013 -->
