# 🧩 LeetCode Problem: Divide Two Integers

- **Problem Link**: [Divide Two Integers – LeetCode](https://leetcode.com/problems/divide-two-integers/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/divide-two-integers/solutions/)

---

## 🧠 Algorithm Explanation

We need to divide two integers without using multiplication, division, or modulus operators.
The algorithm uses bit manipulation (shifts) and subtraction to find how many times the divisor can fit into the dividend.
We keep doubling the divisor (via left shift) to find the largest multiple at each step, which reduces the number of operations from linear to logarithmic.

This approach is optimal because:
- Instead of subtracting `divisor` one at a time (`O(N)`), it subtracts the largest possible multiples (`O(log N)`).
- Works for negative numbers by normalizing the sign and restoring it at the end.
- Handles edge cases like overflow properly.

---

### 🪜 Steps

1. **Handle Edge Cases**:
   - If divisor is `0`, raise an error or return max int.
   - If dividend is `0`, return `0`.

2. **Determine the Sign**:
   - Result is negative if dividend and divisor have opposite signs.

3. **Convert to Absolute Values**:
   - Work with positive integers for simplicity.

4. **Main Loop**:
   - While dividend ≥ divisor:
     - Find the maximum multiple of `divisor` that fits (doubling each time).
     - Subtract that multiple from dividend.
     - Add the corresponding power of two to the result.

5. **Restore Sign & Clamp**:
   - If result should be negative, apply sign.
   - Clamp to the 32-bit signed integer range if necessary.

---

## ✅ Constraints

- Dividend and divisor are integers.
- Divisor ≠ 0.
- Result must fit in a 32-bit signed integer: `[-2³¹, 2³¹ - 1]`.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity   |
|-------------------|--------------|
| Time Complexity   | O(log N)     |
| Space Complexity  | O(1)         |

---
