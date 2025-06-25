# 🧩 LeetCode Problem: Roman to Integer

- **Problem Link**: [Roman to Integer – LeetCode](https://leetcode.com/problems/roman-to-integer/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/roman-to-integer/solutions/)

---

## 🧠 Algorithm Explanation

We convert the Roman numeral string into an integer by scanning it from left to right. Roman numerals use a subtractive rule in certain cases (e.g., `IV` = 4). If a smaller numeral appears before a larger numeral, we subtract it. Otherwise, we add it. This ensures correct handling of all cases.

---

### 🪜 Steps

1. **Create a mapping** of Roman numerals to integers (e.g., `'I': 1, 'V': 5, ..., 'M': 1000`).
2. **Initialize a result variable** to accumulate the final integer value.
3. **Iterate through the string** from left to right:
   - If the current character represents a smaller value than the next character, subtract it.
   - Otherwise, add its value.
4. **Return the accumulated result**.

---

## ✅ Constraints

- `1 <= s.length <= 15`
- `s` contains only the characters `('I', 'V', 'X', 'L', 'C', 'D', 'M')`.
- It is guaranteed that `s` is a valid Roman numeral in the range [1, 3999].

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
