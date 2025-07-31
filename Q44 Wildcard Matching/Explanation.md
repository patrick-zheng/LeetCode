# 🧩 LeetCode Problem: Wildcard Matching

- **Problem Link**: [Wildcard Matching – LeetCode](https://leetcode.com/problems/wildcard-matching/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/wildcard-matching/solutions/)

---

## 🧠 Algorithm Explanation

This problem involves matching a string `s` against a pattern `p` that can include wildcard characters:

- `?` matches any single character
- `*` matches any sequence of characters (including the empty sequence)

To solve this efficiently, we use **dynamic programming (DP)**. We define a 2D boolean matrix `dp`, where `dp[i][j]` indicates whether the first `i` characters of the pattern `p` match the first `j` characters of the string `s`.

This bottom-up DP approach ensures that all subproblems are solved iteratively and avoids redundant recursive calls. It is especially efficient when combined with fast boolean operations and short-circuit evaluation.

---

### 🪜 Steps

1. **Initialization**:
   - Create a DP table of size `(len(p)+1) x (len(s)+1)` with all values as `False`.
   - Set `dp[0][0] = True` (empty pattern matches empty string).
   - Initialize `dp[i][0]` for `p[i-1] == '*'` (star can match empty string).

2. **Filling the DP Table**:
   - Iterate over each character in pattern `p` (rows).
   - For each character in string `s` (columns):
     - If the pattern is `?`, match one character diagonally: `dp[i][j] = dp[i-1][j-1]`
     - If the pattern is `*`, match zero (`dp[i-1][j]`) or more (`dp[i][j-1]`) characters.
     - If the characters match: `dp[i][j] = dp[i-1][j-1]`
     - Otherwise, `dp[i][j] = False`.

3. **Final Result**:
   - Return the value of `dp[len(p)][len(s)]`.

---

## ✅ Constraints

- `1 <= s.length, p.length <= 2000`
- `s` contains only lowercase English letters.
- `p` contains only lowercase English letters, `?`, or `*`.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity      |
|-------------------|-----------------|
| Time Complexity   | O(m × n)        |
| Space Complexity  | O(m × n)        |

Where `m = len(p)` and `n = len(s)`.

> Note: This can be further optimized to O(n) space using a rolling array technique.

---
