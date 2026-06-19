# LeetCode Problem: Longest Increasing Subsequence

- **Problem Link**: [Longest Increasing Subsequence – LeetCode](https://leetcode.com/problems/longest-increasing-subsequence/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/longest-increasing-subsequence/solutions/)

---

## Algorithm

Return the length of the longest **strictly** increasing subsequence of
`nums` (elements need not be contiguous).

### Patience sorting with a `tails` array (**O(n log n)**)

Maintain `tails` where `tails[len - 1]` is the **smallest possible tail**
among all increasing subsequences of length `len`. The array stays sorted,
so its length is the answer.

For each `num`:

1. Binary-search the leftmost index `pos` with `tails[pos] >= num`
   (`bisect_left` / `lower_bound` / `partition_point`).
2. If `pos == len(tails)`, append `num` (we extended the best length).
3. Otherwise set `tails[pos] = num` (same length, smaller tail — better for
   future extensions).

Example: `nums = [10, 9, 2, 5, 3, 7, 101, 18]` → answer `4`
(subsequence `[2, 3, 7, 101]`).

### Why it works

Replacing a tail with a smaller value keeps the subsequence length but makes
it easier to append larger numbers later. Each element does one binary search
on an array of size at most `n`, so total time is **O(n log n)**.

---

## Constraints

- `1 <= nums.length <= 2500`
- `-10^4 <= nums[i] <= 10^4`

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n log n)** — one binary search per element on `tails`                       |
| Space Complexity   | **O(n)** — `tails` holds at most one candidate per subsequence length           |

---
<!-- markdownlint-enable MD013 -->
