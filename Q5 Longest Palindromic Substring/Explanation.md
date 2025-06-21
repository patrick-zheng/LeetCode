# 🧩 LeetCode Problem: Longest Palindromic Substring

- **Problem Link**: [Longest Palindromic Substring – LeetCode](https://leetcode.com/problems/longest-palindromic-substring/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/longest-palindromic-substring/solutions/)

---

## 🧠 Algorithm Explanation

This problem is solved using **Manacher’s Algorithm**, which finds the **longest palindromic substring** in **O(n)** time.

### Why Manacher’s Algorithm?

- It improves upon the common expand-around-center approach (which takes O(n²)) by avoiding redundant checks through the use of symmetry.
- It works by transforming the string so all palindromes are odd-length, then expanding around centers efficiently.

---

### 🪜 Steps

1. **Preprocess** the string by inserting `#` between every character and surrounding it with boundary characters `^` and `$`.  
   Example: `"abba"` → `"^#a#b#b#a#$"`

2. Create an array `P` where `P[i]` holds the **radius** of the longest palindrome centered at index `i` in the transformed string.

3. Maintain two variables:
   - `center`: the center of the current rightmost palindrome.
   - `right`: the right boundary of that palindrome.

4. For each character `i` in the transformed string:
   - Let `mirror = 2 * center - i`
   - If `i < right`, initialize `P[i] = min(P[mirror], right - i)`
   - Try to **expand** around `i` while characters on both sides match.
   - If the palindrome around `i` expands past `right`, update `center` and `right`.

5. Find the **maximum value in `P`**, and use its center to extract the longest palindrome in the original string.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |

---
