# 🧩 LeetCode Problem: 190. Reverse Bits

- **Problem Link**: [Reverse Bits – LeetCode](https://leetcode.com/problems/reverse-bits/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/reverse-bits/solutions/)

---

## 🧠 Algorithm Explanation

The problem asks us to reverse the bits of a given 32-bit unsigned integer.

Since the integer always contains exactly **32 bits**, we can iterate through all 32 positions.  
At each step:

- Extract the **least significant bit (LSB)** using `n & 1`
- Shift the result left to make room for the new bit
- Append the extracted bit
- Shift the original number right to process the next bit

Because the number of bits is fixed (32), this is effectively a **constant-time operation**.

This approach is optimal because:

- It avoids string conversion
- It uses direct bitwise manipulation
- It runs in constant time and space

---

### 🪜 Steps

1. **Initialize result to 0**
   - This will store the reversed bits.

2. **Iterate 32 times**
   - Extract the last bit of `n` using `n & 1`.
   - Shift `result` left by 1.
   - Add the extracted bit to `result`.
   - Shift `n` right by 1.

3. **Return result**
   - After 32 iterations, all bits are reversed.

---

## ✅ Constraints

- The input is a 32-bit unsigned integer.
- The integer may be signed in some languages, but the bit pattern remains the same.
- Always exactly 32 bits.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(1)       |
| Space Complexity  | O(1)       |

Since the loop runs exactly 32 times regardless of input size, the complexity is constant.

---
