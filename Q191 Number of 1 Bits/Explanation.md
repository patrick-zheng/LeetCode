# 🧩 LeetCode Problem: 191. Number of 1 Bits

- **Problem Link**: [Number of 1 Bits – LeetCode](https://leetcode.com/problems/number-of-1-bits/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/number-of-1-bits/solutions/)

---

## 🧠 Algorithm Explanation

This problem asks us to compute the **Hamming Weight**, which is the number of `1` bits in the binary representation of a number.

The optimal approach uses **Brian Kernighan’s algorithm**. Instead of checking every bit individually, this method repeatedly removes the **lowest set bit** using:

---

### 🪜 Steps

1. Initialize `count = 0`.
2. While `n` is not `0`:
   - Set `n = n & (n - 1)` to remove the lowest set bit.
   - Increment `count`.
3. Return `count`.

---

## ✅ Constraints

- `0 <= n <= 2³² - 1`
- The integer is treated as **unsigned** in most implementations.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity                        |
|-------------------|------------                       |
| Time Complexity   | O(k) where k = number of set bits |
| Space Complexity  | O(1)                              |

---
