# 🧩 LeetCode Problem: Regular Expression Matching

- **Problem Link**: [Regular Expression Matching – LeetCode](https://leetcode.com/problems/regular-expression-matching/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/regular-expression-matching/solutions/)

---

## 🧠 Algorithm Explanation

We use **dynamic programming (DP)** to determine if the input string `s` matches the pattern `p` according to the given rules:

- `.` matches any single character.
- `*` matches zero or more of the preceding element.

We define `dp[i][j]` as a boolean representing whether `s[:i]` matches `p[:j]`. The key insight is that `dp[i][j]` can be built using previously computed subproblems.

---

### 🪜 Steps

1. **Initialize DP Table**:
   - Create a `(m+1) x (n+1)` 2D boolean array `dp` where `m = len(s)` and `n = len(p)`.
   - Set `dp[0][0] = True`, meaning an empty string matches an empty pattern.
   - Pre-fill the first row for cases where `p` contains `*` that could eliminate preceding characters (like `"a*"`).

2. **Fill DP Table**:
   - Loop through `i = 1 to m` and `j = 1 to n`.
   - If `p[j - 1] == s[i - 1]` or `p[j - 1] == '.'`, set `dp[i][j] = dp[i - 1][j - 1]`.
   - If `p[j - 1] == '*'`, evaluate:
     - **Zero occurrence**: `dp[i][j] = dp[i][j - 2]`
     - **One or more**: if `s[i - 1] == p[j - 2]` or `p[j - 2] == '.'`, also include `dp[i - 1][j]`.

3. **Return Result**:
   - The final result is stored in `dp[m][n]`.

---

## ✅ Constraints

- `1 <= s.length <= 20`
- `1 <= p.length <= 20`
- `s` contains only lowercase English letters.
- `p` contains only lowercase English letters, `'.'`, and `'*'`.
- It is guaranteed that for every `'*'`, there is a preceding valid character to match.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity   |
|-------------------|--------------|
| Time Complexity   | O(m × n)     |
| Space Complexity  | O(m × n)     |

Where `m` is the length of the input string `s`, and `n` is the length of the pattern `p`.

---
