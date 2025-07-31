# 🧩 LeetCode Problem: Multiply Strings

- **Problem Link**: [Multiply Strings – LeetCode](https://leetcode.com/problems/multiply-strings/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/multiply-strings/solutions/)

---

## 🧠 Algorithm Explanation

This problem simulates multiplication of two large numbers represented as strings. Since converting the strings directly to integers is not allowed, we use digit-wise multiplication similar to the manual multiplication method taught in school.

The core idea is to simulate the multiplication of each digit in `num1` with each digit in `num2` and store the results at the correct positions in a result array. Each index in this array corresponds to a power of ten, and we properly manage carries during the multiplication and summation phases.

---

### 🪜 Steps

1. **Edge Case Check**:
   - If either `num1` or `num2` is `"0"`, return `"0"` immediately.

2. **Initialize Result Array**:
   - Create a result array of size `len(num1) + len(num2)` initialized with zeros to store the product.

3. **Digit-by-Digit Multiplication**:
   - Iterate over `num1` and `num2` in reverse (from least to most significant digit).
   - Multiply digits and accumulate them at the correct position in the result array using carry management.

4. **Handle Carry and Convert to String**:
   - After all multiplications, convert the result array to a string, stripping any leading zeros.

---

## ✅ Constraints

- `1 <= num1.length, num2.length <= 200`
- `num1` and `num2` consist of digits only.
- Both `num1` and `num2` do not contain any leading zero, except the number "0" itself.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity      |
|-------------------|-----------------|
| Time Complexity   | O(n * m)        |
| Space Complexity  | O(n + m)        |

Where `n` is the length of `num1` and `m` is the length of `num2`.

---
