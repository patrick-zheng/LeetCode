# LeetCode Problem: Missing Number

- **Problem Link**: [Missing Number – LeetCode](https://leetcode.com/problems/missing-number/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/missing-number/solutions/)

---

## Algorithm

`nums` has length `n` and contains **n** distinct integers from `0` to `n`
with **exactly one** missing. Brute force sorting or a hash set is
**O(n log n)** or **O(n)** extra space.

### XOR cancellation

XOR every index `0 .. n` and every value in `nums`. Each present value
`k` appears once as an index and once in the array, so `k ^ k = 0`. The
missing value appears only once (as an index or only in `nums`), so it
remains in the result.

Initialize `missing = n`, then for each `i` and `nums[i]`:

`missing ^= i ^ nums[i]`

This also covers the case where the missing number is `n` (never appears
as an array element, only as the initial `missing = n`).

Equivalent: XOR all `i` in `0..=n`, then XOR all `nums[i]`; same result.

---

## Constraints

- `n == nums.length`
- `1 <= n <= 10^4`
- `0 <= nums[i] <= n`
- All entries in `nums` are **unique**

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** — one pass over `nums`                                                 |
| Space Complexity   | **O(1)** — only the XOR accumulator                                             |

---
<!-- markdownlint-enable MD013 -->
