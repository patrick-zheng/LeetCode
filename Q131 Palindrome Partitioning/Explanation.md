# 🧩 LeetCode Problem: Palindrome Partitioning

- **Problem Link**: [Palindrome Partitioning – LeetCode](https://leetcode.com/problems/palindrome-partitioning/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/palindrome-partitioning/solutions/)

---

## 🧠 Algorithm Explanation

We want to split the string `s` into substrings such that **every substring is a palindrome**, and return **all possible** such partitions.

This is naturally solved using **backtracking**:

- At index `start`, we try all possible substrings `s[start..end]`.
- If `s[start..end]` is a **palindrome**, we:
  - Add it to the current partition (`path`),
  - Recurse from `end + 1`,
  - Then backtrack (remove the last added substring).
- When `start == s.length()`, we have a complete valid partition and add it to the result.

We use a helper `isPalindrome` to check if a substring is a palindrome by comparing characters from both ends.

Backtracking is used because we need **all combinations** of valid partitions, and each step depends on local validity (palindrome).

---

### 🪜 Steps

1. **Initialize containers**  
   - Create `res` to store all valid partitions: `vector<vector<string>>`.
   - Create `path` to store the current partition: `vector<string>`.

2. **Backtracking function**  
   - Implement `backtrack(start)`:
     - If `start == s.size()`, push `path` into `res` and return.
     - Otherwise, for every `end` from `start` to `s.size() - 1`:
       - If `s[start..end]` is a palindrome:
         - Push `s.substr(start, end - start + 1)` onto `path`.
         - Call `backtrack(end + 1)`.
         - Pop the last string from `path` (backtrack).

3. **Palindrome check**  
   - Implement `isPalindrome(s, left, right)`:
     - While `left < right`:
       - If `s[left] != s[right]`, return `false`.
       - Increment `left`, decrement `right`.
     - If the loop finishes, return `true`.

---

## ✅ Constraints

- `1 <= s.length <= 16`
- `s` consists of only lowercase English letters.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity          |
|-------------------|---------------------|
| Time Complexity   | O(n² · 2ⁿ) (worst) |
| Space Complexity  | O(n) (recursion + path) |

- There can be up to **O(2ⁿ)** partitions for a string of length `n`.
- Each palindrome check can cost up to **O(n)**.
- Overall time is **O(n² · 2ⁿ)** in the worst case.
- Extra space is for recursion depth and current path, at most `O(n)`.

---
