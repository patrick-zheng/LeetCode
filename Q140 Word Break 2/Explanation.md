# 🧩 LeetCode Problem: Word Break II

- **Problem Link**: [Word Break II – LeetCode](https://leetcode.com/problems/word-break-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/word-break-ii/solutions/)

---

## 🧠 Algorithm Explanation

We need **all** valid sentences, not just whether one exists.  
A clean approach is:

1. **DFS (backtracking)** to build sentences by trying every dictionary word that matches the current prefix.
2. **Memoization** so we don’t recompute “all sentences starting at index `i`” multiple times.

Let `dfs(i)` return all sentences you can form from `s[i..]`.  
Then for every word `w` that matches `s[i..i+len(w))`, we combine:

- `w` alone if it reaches the end, or
- `w + " " + tail` for each `tail` in `dfs(i+len(w))`.

Memoize `dfs(i)`.

---

### 🪜 Steps

1. Put `wordDict` into a `HashSet<String>` for fast membership checks.
2. Precompute `max_len` = maximum word length in the dictionary (so we don’t test impossible lengths).
3. Define `dfs(i)` with memoization:
   - If `i == n`, return `[""]` (empty sentence tail).
   - Try all `j` from `i+1` to `min(n, i+max_len)`:
     - If `s[i..j]` is a dictionary word, get all tails from `dfs(j)` and combine.
4. Return `dfs(0)`.

---

## ✅ Constraints

- `1 <= s.length <= 20` (small enough to generate all answers, but still exponential in worst case)
- `1 <= wordDict.length <= 1000`
- Words can be reused

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(#answers * n) (output-dominated) |
| Space Complexity  | O(#answers * n) + O(n) memo keys |

---
