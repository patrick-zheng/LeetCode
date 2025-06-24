# 🧩 LeetCode Problem: String to Integer (atoi)

- **Problem Link**: [String to Integer (atoi) – LeetCode](https://leetcode.com/problems/string-to-integer-atoi/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/string-to-integer-atoi/solutions/)

---

## 🧠 Algorithm Explanation

The algorithm simulates the process of converting a string to an integer, closely mimicking the behavior of the `atoi` function in C/C++.  
It handles leading whitespace, optional sign characters (`+` or `-`), numeric digits, and early termination upon encountering any non-digit characters.  
It also clamps the final result within the 32-bit signed integer range: [−2³¹, 2³¹ − 1].

This approach was chosen for its linear runtime and direct, readable logic that matches problem constraints and edge cases effectively.

---

### 🪜 Steps

1. **Trim Whitespace**: Remove leading whitespace using `lstrip()`.
2. **Parse Sign**: Check if the first character is `'+'` or `'-'` and record the sign.
3. **Convert Digits**: Traverse the remaining characters while they are digits and construct the number.
4. **Clamp Result**: Ensure the final result lies within the 32-bit signed integer range using `max()` and `min()`.

---

## Constraints

- 0 <= s.length <= 200
- s consists of English letters (lower-case and upper-case), digits (0-9), ' ', '+', '-', and '.'

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

Where `n` is the length of the input string.

---
