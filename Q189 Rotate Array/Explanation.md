# 🧩 LeetCode Problem: 189. Rotate Array

- **Problem Link**: [Rotate Array – LeetCode](https://leetcode.com/problems/rotate-array/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/rotate-array/solutions/)

---

## 🧠 Algorithm Explanation

To rotate an array to the right by `k` steps efficiently and **in-place**, we use the **three-reversal technique**.

### Why this algorithm?

A right rotation by `k` means:

- The last `k` elements move to the front.
- The first `n - k` elements shift to the right.

Instead of shifting elements one by one (which would be `O(n·k)`), we can achieve this in **O(n)** time and **O(1)** extra space using reversals:

1. Reverse the entire array.
2. Reverse the first `k` elements.
3. Reverse the remaining `n - k` elements.

This rearranges elements into the desired rotated order.

---

### 🪜 Steps

1. Compute effective rotation
2. Reverse the entire array.
3. Reverse the first `k` elements.
4. Reverse the remaining `n - k` elements.

---

## ✅ Constraints

- `1 <= nums.length <= 10^5`
- `-2^31 <= nums[i] <= 2^31 - 1`
- `0 <= k <= 10^5`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
