# 🧩 LeetCode Problem: Excel Sheet Column Number

- **Problem Link**: [Excel Sheet Column Number – LeetCode](https://leetcode.com/problems/excel-sheet-column-number/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/excel-sheet-column-number/solutions/)

---

## 🧠 Algorithm Explanation

Excel column titles follow a **base-26 numbering system** using uppercase letters instead of digits.  
Each letter maps to a number:

- `A → 1`, `B → 2`, …, `Z → 26`

Unlike normal base systems, there is **no zero digit**, which means we must always add `1` when converting characters.

The idea is to iterate through the string from **left to right**, treating the result like a number being built in base 26:

- Multiply the current result by 26
- Add the numeric value of the current character

This approach is efficient and works because column titles behave exactly like positional notation.

---

### 🪜 Steps

1. **Initialize a result variable** to `0`
2. **Iterate through each character** in `columnTitle`
   - Convert the character to its numeric value using  
     `value = char - 'A' + 1`
   - Update the result:  
     `result = result * 26 + value`
3. **Return the final result** after processing all characters

---

## ✅ Constraints

- `1 <= columnTitle.length <= 7`
- `columnTitle` consists only of uppercase English letters
- The result fits within a 32-bit signed integer

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)**   |
| Space Complexity  | **O(1)**   |

Where `n` is the length of the column title.

---
