# 🧩 LeetCode Problem: Valid Palindrome

- **Problem Link**: [Valid Palindrome – LeetCode](https://leetcode.com/problems/valid-palindrome/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/valid-palindrome/solutions/)

---

## 🧠 Algorithm Explanation

We want to check if a string is a palindrome after:

- Converting all uppercase letters to lowercase, and  
- Removing all non-alphanumeric characters (anything that is not `a–z`, `A–Z`, or `0–9`).

To do this efficiently, we use a **two-pointer** approach:

- One pointer starts at the **beginning** (`left`), the other at the **end** (`right`).
- Move both pointers toward the center:
  - Skip characters that are **not alphanumeric**.
  - Compare the lowercase versions of the characters at `left` and `right`.
- If at any point the characters differ, the string is **not** a palindrome.
- If we reach the middle without mismatch, the string **is** a palindrome.

This avoids extra memory for a cleaned string and runs in linear time.

---

### 🪜 Steps

1. **Initialize pointers**  
   Set `left = 0` and `right = len(s) - 1`.

2. **Skip non-alphanumeric characters**  
   While `left < right` and `s[left]` is not alphanumeric, increment `left`.  
   While `left < right` and `s[right]` is not alphanumeric, decrement `right`.

3. **Compare characters (case-insensitive)**  
   - Compare `lower(s[left])` and `lower(s[right])`.  
   - If they differ → return `false`.  
   - Otherwise, move inward: `left += 1`, `right -= 1`.

4. **Return result**  
   If the loop finishes without mismatches, return `true`.

---

## ✅ Constraints

- `1 <= s.length <= 2 * 10^5`
- `s` may contain letters, digits, spaces, and punctuation.
- Only letters `a–z`, `A–Z` and digits `0–9` are considered alphanumeric.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | `O(n)`     |
| Space Complexity  | `O(1)`     |

---
