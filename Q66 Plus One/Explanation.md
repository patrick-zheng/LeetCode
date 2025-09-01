# 🧩 LeetCode Problem: Plus One

- **Problem Link**: [Plus One – LeetCode](https://leetcode.com/problems/plus-one/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/plus-one/solutions/)

---

## 🧠 Algorithm Explanation

Treat the array `digits` as a big integer and add `1` starting from the least significant digit (rightmost). Maintain a `carry` initialized to `1`. For each digit from right to left, compute the new digit with modulo (`% 10`) and update the carry with integer division (`// 10`).  
If the carry becomes `0` at any point, you can stop early because higher-order digits remain unchanged. If a carry remains after processing all digits (e.g., input like `[9,9,9]`), insert `1` at the front.

---

### 🪜 Steps

1. **Initialize**: Set `carry = 1` and let `n = len(digits)`.
2. **Propagate from right**: For `i` from `n-1` down to `0`:
   - `total = digits[i] + carry`
   - `digits[i] = total % 10`, `carry = total // 10`
   - If `carry == 0`, break early.
3. **Handle leftover carry**: If `carry == 1` after the loop, insert `1` at the beginning.

---

## ✅ Constraints

- `1 <= digits.length <= 100`
- `0 <= digits[i] <= 9`
- No leading zeros unless the number itself is `0` (i.e., `[0]` is valid).

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1) extra |

---
