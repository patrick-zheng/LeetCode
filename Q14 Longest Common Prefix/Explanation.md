# 🧩 LeetCode Problem: Longest Common Prefix

- **Problem Link**: [14. Longest Common Prefix – LeetCode](https://leetcode.com/problems/longest-common-prefix/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/longest-common-prefix/solutions/)

---

## 🧠 Algorithm Explanation

We use the **vertical scanning** approach. In this method, we iterate character by character across all strings at the same index. As soon as a mismatch is found or we reach the end of any string, we stop and return the prefix found so far.

This approach is efficient in practice and avoids unnecessary processing beyond the point of divergence.

An **alternative approach** is to sort the strings and compare only the **first and last** strings — the two that differ the most after sorting.

---

### 🪜 Steps

1. **Check if the input list is empty**:
   - Return an empty string if so.

2. **Iterate through characters of the first string**:
   - Use the characters of the first string as a reference.
   - For each character index, compare with the same index of every other string.

3. **Break early** if:
   - Any string ends, or
   - A mismatch is found.

4. **Return the prefix** up to the index where all matched.

---

## ✅ Constraints

- `1 <= strs.length <= 200`
- `0 <= strs[i].length <= 200`
- `strs[i]` consists of only lowercase English letters.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(S)       |
| Space Complexity  | O(1)       |

Where `S` is the sum of all characters in all strings.

---
