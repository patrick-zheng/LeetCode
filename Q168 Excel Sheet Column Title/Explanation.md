# 🧩 LeetCode Problem: Excel Sheet Column Title

- **Problem Link**: [Excel Sheet Column Title – LeetCode](https://leetcode.com/problems/excel-sheet-column-title/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/excel-sheet-column-title/solutions/)

---

## 🧠 Algorithm Explanation

Excel column titles follow a **base-26 system without a zero digit**.  
Instead of digits `0–25`, Excel uses letters `A–Z` mapped to values `1–26`.

This means we cannot directly apply normal base conversion.  
To fix this, we **subtract 1** from the column number before taking the modulo.  
This adjustment aligns the values so that:

- `A → 0`
- `Z → 25`

We repeatedly extract the last letter using modulo, convert it to a character,
and divide the number by 26 until it becomes zero.

---

### 🪜 Steps

1. **Convert to 0-based index**  
   Subtract `1` from `columnNumber` to handle the lack of a zero digit.

2. **Extract current character**  
   Compute `columnNumber % 26` and map it to a letter using `'A' + remainder`.

3. **Build result and continue**  
   Append the character, divide `columnNumber` by `26`, and repeat.  
   Reverse the collected characters at the end.

---

## ✅ Constraints

- `1 <= columnNumber <= 2^31 - 1`
- Input is always a valid positive integer

---

## ⏱ Time and Space Complexity

| Metric            | Complexity        |
|-------------------|-------------------|
| Time Complexity   | `O(log₍26₎ n)`    |
| Space Complexity  | `O(log₍26₎ n)`    |

---
