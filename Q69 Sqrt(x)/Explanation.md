# 🧩 LeetCode Problem: Sqrt(x)

- **Problem Link**: [Sqrt(x) – LeetCode](https://leetcode.com/problems/sqrtx/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/sqrtx/solutions/)

---

## 🧠 Algorithm Explanation

We need to find the square root of a non-negative integer `x`, rounded down to the nearest integer, without using built-in exponentiation functions.  
A binary search works efficiently because the function `f(m) = m²` is monotonic (increasing).  

We search between `2` and `x/2` (since for `x ≥ 2`, the integer square root is always ≤ `x/2`).  
At each step, we check if `mid²` is greater than, less than, or equal to `x`:

- If `mid² == x`, return `mid`.
- If `mid² < x`, move `left` up (`left = mid + 1`).
- If `mid² > x`, move `right` down (`right = mid - 1`).  

When the loop ends, `right` will hold the floor value of `sqrt(x)`.

---

### 🪜 Steps

1. **Handle small cases**: If `x < 2`, just return `x`.
2. **Initialize search range**: Set `left = 2`, `right = x // 2`.
3. **Binary search loop**:
   - Compute `mid = left + (right - left) // 2`.
   - Compare `mid²` with `x`.
   - Adjust `left` or `right` accordingly.
4. **Return result**: When the loop finishes, return `right` as the integer square root.

---

## ✅ Constraints

- \(0 \leq x \leq 2^{31} - 1\)  
- Answer must fit in a **32-bit signed integer**.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | \(O(\log x)\) |
| Space Complexity  | \(O(1)\)    |

---
