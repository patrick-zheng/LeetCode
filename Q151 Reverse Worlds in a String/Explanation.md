# 🧩 LeetCode Problem: Reverse Words in a String

- **Problem Link**: [Reverse Words in a String – LeetCode](https://leetcode.com/problems/reverse-words-in-a-string/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/reverse-words-in-a-string/solutions/)

---

## 🧠 Algorithm Explanation

The goal is to reverse the **order of words** in a given string while removing extra spaces.  
A *word* is defined as a sequence of non-space characters.

Python provides built-in string utilities that make this problem straightforward:

- `split()` automatically handles multiple spaces and trims leading/trailing spaces
- List slicing can reverse the word order efficiently
- `join()` reconstructs the final string with proper spacing

This approach is chosen because it is **simple, readable, and efficient**, avoiding manual parsing.

---

### 🪜 Steps

1. **Split the string into words**  
   Use `split()` to separate words based on whitespace and discard extra spaces.

2. **Reverse the list of words**  
   Use list slicing `[::-1]` to reverse the order.

3. **Join the words into a string**  
   Use `' '.join()` to combine the reversed words with single spaces.

---

## ✅ Constraints

- The input string `s` may contain leading or trailing spaces
- Multiple spaces may exist between words
- Words consist of non-space characters only
- Output must contain words separated by a **single space only**

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |

Where `n` is the length of the input string.

---
