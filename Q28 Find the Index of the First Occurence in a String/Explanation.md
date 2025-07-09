# 🧩 LeetCode Problem: Find the Index of the First Occurrence in a String

- **Problem Link**: [Find the Index of the First Occurrence in a String – LeetCode](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/solutions/)

---

## 🧠 Algorithm Explanation

We use the **Knuth-Morris-Pratt (KMP)** algorithm for efficient substring search.
KMP is optimal because it avoids re-examining characters in the `haystack` that have already been matched.
It preprocesses the `needle` to build a longest-prefix-suffix (LPS) array which allows the search to skip unnecessary comparisons.

---

### 🪜 Steps

1. **Build the LPS array**:
   For the `needle`, construct an array that tells for each position the length of the longest prefix which is also a suffix.

2. **Search with LPS guidance**:
   Iterate over the `haystack` and use the LPS array to skip ahead intelligently when a mismatch occurs.

3. **Return index or -1**:
   If the full `needle` is found in the `haystack`, return the start index of the match. Otherwise, return `-1`.

---

## ✅ Constraints

- `1 <= haystack.length, needle.length <= 10⁴`
- `haystack` and `needle` consist of only lowercase English characters.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(N + M)** |
| Space Complexity  | **O(M)** |

Where:
- \( N \) = length of `haystack`
- \( M \) = length of `needle`

---
