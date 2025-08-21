# 🧩 LeetCode Problem: Length of Last Word

- **Problem Link**: [Length of Last Word – LeetCode](https://leetcode.com/problems/length-of-last-word/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/length-of-last-word/solutions/)

---

## 🧠 Algorithm Explanation

The goal is to return the length of the last word in a given string `s`.  
A word is defined as a maximal substring consisting of non-space characters only.

### Why this algorithm?

- We first **trim leading and trailing spaces** to remove unnecessary whitespace.
- Then we **split the string by spaces** into words.
- The **last element in the list** is guaranteed to be the last word (since trailing spaces were removed).
- Return the length of this last word.

This approach is straightforward, readable, and leverages Python’s built-in string methods.

---

### 🪜 Steps

1. **Trim spaces**: Remove leading and trailing spaces using `strip()`.
2. **Check empty string**: If the result is empty, return 0.
3. **Split into words**: Use `split(" ")` to separate words by spaces.
4. **Get last word**: Select the last element `words[-1]`.
5. **Return its length**: Use `len()`.

---

## ✅ Constraints

- `1 <= s.length <= 10^4`
- `s` consists of only English letters and spaces `' '`.
- There will be at least one word in `s`.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |

- **Time**: We traverse the string once for trimming and splitting.  
- **Space**: Splitting creates an array of words proportional to the input size.  

---
