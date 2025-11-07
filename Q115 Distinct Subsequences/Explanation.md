# 🧩 LeetCode Problem: Distinct Subsequences

- **Problem Link**: [Distinct Subsequences – LeetCode](https://leetcode.com/problems/distinct-subsequences/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/distinct-subsequences/solutions/)

---

## 🧠 Algorithm Explanation

We want the number of ways the target string `t` appears as a **subsequence** of `s`.  
Define `dp[j]` as the number of ways to form the first `j` characters of `t` from the prefix of `s` processed so far. Initialize `dp[0] = 1` (empty target is always possible once).

As we scan each character `ch` in `s`, we update `dp` **right-to-left** so each `s` character is used at most once per state transition:

- If `ch == t[j-1]`, then any way to make `t[:j-1]` can extend to `t[:j]`: `dp[j] += dp[j-1]`.

This yields an `O(m·n)` time, `O(n)` space dynamic program (with `m = len(s)`, `n = len(t)`).

---

### 🪜 Steps

1. **Init**: Create `dp` of length `n+1` with `dp[0] = 1`, others `0`.
2. **Iterate `s`**: For each `ch` in `s`, loop `j` from `n` down to `1`.
3. **Transition**: If `ch == t[j-1]`, do `dp[j] += dp[j-1]`.
4. **Result**: Return `dp[n]`.

---

## ✅ Constraints

- `1 ≤ len(s), len(t) ≤ 1000` (typical).
- Result fits in a **32-bit signed integer** (guaranteed by test cases).
- Edge cases:
  - `t == ""` → `1`
  - `len(s) < len(t)` → `0`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity     |
|-------------------|----------------|
| Time Complexity   | `O(m · n)`     |
| Space Complexity  | `O(n)`         |

---
