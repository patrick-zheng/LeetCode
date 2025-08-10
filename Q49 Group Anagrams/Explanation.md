# 🧩 LeetCode Problem: Group Anagrams

- **Problem Link**: [Group Anagrams – LeetCode](https://leetcode.com/problems/group-anagrams/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/group-anagrams/solutions/)

---

## 🧠 Algorithm Explanation

The problem requires grouping strings that are anagrams of each other.  
Anagrams share the same character frequency, so we can represent each string by its character count signature (a tuple of counts of each letter). Using this signature as a key, group all strings that share the same key.

This approach is efficient because it avoids sorting each string (which is more expensive) and instead counts characters in O(K) time per string, where K is the string length.

---

### 🪜 Steps

1. **Initialize** a hashmap/dictionary to map the character count tuple to a list of strings.
2. For each string:
   - Count the frequency of each character (assuming lowercase a-z).
   - Convert the frequency list to a tuple and use it as the key.
   - Append the string to the list corresponding to this key.
3. Return all grouped anagrams as a list of lists.

---

## ✅ Constraints

- All strings consist of lowercase English letters.
- Number of strings and length of each string can vary (up to problem limits).
- Return groups in any order.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity    |
|-------------------|---------------|
| Time Complexity   | O(N * K)      |
| Space Complexity  | O(N * K)      |

- N = number of strings
- K = max length of a string

---
