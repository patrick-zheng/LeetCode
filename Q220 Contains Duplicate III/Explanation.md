# LeetCode Problem: Contains Duplicate III

- **Problem Link**: [Contains Duplicate III – LeetCode](https://leetcode.com/problems/contains-duplicate-iii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/contains-duplicate-iii/solutions/)

---

## Algorithm

We need **distinct indices** `i` and `j` with `|i - j| <= k` and `|nums[i] - nums[j]| <= t`.

A brute force check of all pairs in each window is **O(n·k)** and is too slow when `k` is large. Sorting each window is also too heavy.

### Bucket + sliding window (average **O(n)**)

Partition the number line into buckets of width **`t + 1`**. Any two integers that fall in the **same** bucket differ by at most **`t`**. If two numbers are in **adjacent** buckets, they might still differ by at most **`t`**, so we must compare with the stored values in `bucket - 1` and `bucket + 1`.

Maintain only the last **`k + 1`** indices (sliding window). For each new value:

1. If the window already has more than `k` elements, remove the bucket entry for the element leaving the window (the value at index `i - k - 1`).
2. Compute `bucket = floor(nums[i] / (t + 1))` using **signed** floor division so negatives map correctly (same rule in C++ via `(num + 1) / width - 1` for `num < 0`).
3. If `bucket` is already present → **true** (same bucket ⇒ difference ≤ `t`).
4. If `bucket - 1` exists and `nums[i] - stored ≤ t` → **true**.
5. If `bucket + 1` exists and `stored - nums[i] ≤ t` → **true**.
6. Store `nums[i]` in `bucket`.

At most one value per bucket is needed inside the window: two distinct values in the same bucket already imply the answer.

Use **`i64` / `long long`** for bucket ids and differences so `t` and `nums[i]` near `INT_MIN`/`INT_MAX` do not overflow.

---

## Constraints

- `1 <= nums.length <= 2 * 10^5`
- `-2^31 <= nums[i] <= 2^31 - 1`
- `0 <= k <= 10^5`
- `0 <= t <= 2^31 - 1`

---

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** average — each index does **O(1)** map work; hashmap expected constant |
| Space Complexity   | **O(min(n, k + 1))** — at most one bucket per value in the window               |

---
