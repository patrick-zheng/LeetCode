# LeetCode Problem: Range Sum Query - Immutable

- **Problem Link**: [Range Sum Query - Immutable – LeetCode](https://leetcode.com/problems/range-sum-query-immutable/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/range-sum-query-immutable/solutions/)

---

## Algorithm

Design `NumArray` so `sumRange(left, right)` returns the sum of `nums[left..right]`
in **O(1)** time after preprocessing. The array never changes.

### Prefix sums

Build `prefix` with length `n + 1`:

- `prefix[0] = 0`
- `prefix[i + 1] = prefix[i] + nums[i]`

Then:

$$
\text{sumRange}(left, right) = prefix[right + 1] - prefix[left]
$$

`prefix[k]` is the sum of the first `k` elements of `nums`, so subtracting
removes everything before `left` and keeps through `right`.

Example: `nums = [-2, 0, 3, -5, 2, -1]`

- `sumRange(0, 2)` → `prefix[3] - prefix[0]` → `1`
- `sumRange(2, 5)` → `prefix[6] - prefix[2]` → `-1`

---

## Constraints

- `1 <= nums.length <= 10^4`
- `-10^5 <= nums[i] <= 10^5`
- `0 <= left <= right < nums.length`
- At most `10^4` calls to `sumRange`

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** build prefix once; each `sumRange` is **O(1)**                         |
| Space Complexity   | **O(n)** — prefix array of length `n + 1`                                       |

---
<!-- markdownlint-enable MD013 -->
