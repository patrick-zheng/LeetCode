# 🧩 LeetCode Problem: Add Binary

- **Problem Link**: [Add Binary – LeetCode](https://leetcode.com/problems/add-binary/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/add-binary/solutions/)

---

## 🧠 Algorithm Explanation

We are asked to add two binary strings and return their sum as a binary string.  
The approach is similar to manual binary addition: start from the least significant digit (rightmost), add corresponding bits along with any carry, and move left until both strings are fully processed. If a carry remains after processing all digits, append it at the end.

This algorithm is used because it directly mimics binary addition rules and works efficiently in linear time without requiring conversions to integers, which may cause overflow for very large inputs.

---

### 🪜 Steps

1. **Initialize pointers and carry**:  
   - Start from the last indices of both strings.  
   - Keep a `carry` variable initialized to `0`.  

2. **Iterate while digits or carry remain**:  
   - Extract bits from `a` and `b`, or treat them as `0` if the pointer is out of bounds.  
   - Compute `total = bit_a + bit_b + carry`.  
   - Append `total % 2` to the result list.  
   - Update `carry = total // 2`.  

3. **Finalize the result**:  
   - Reverse the result list (since digits were added from right to left).  
   - Join into a string and return it.  

---

## ✅ Constraints

- `1 <= a.length, b.length <= 10^4`  
- Each string consists only of `'0'` or `'1'`.  
- Strings do not contain leading zeros except for `"0"`.  

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(max(n, m)) where n = len(a), m = len(b) |
| Space Complexity  | O(max(n, m)) for storing the result |

---
