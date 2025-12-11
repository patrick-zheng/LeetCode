# 🧩 LeetCode Problem: Word Break

- **Problem Link**: [Word Break – LeetCode](https://leetcode.com/problems/word-break/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/word-break/solutions/)

---

## 🧠 Algorithm Explanation

We want to know if the string `s` can be split into a sequence of words from `wordDict`.  

A natural way to think about this is:

> “Can I cut `s` somewhere so that the left part is valid (can be segmented) and the right part is a dictionary word?”

This leads directly to **dynamic programming**:

- Let `dp[i]` mean:  
  > `true` if the substring `s[0..i)` (first `i` characters) can be segmented into dictionary words.
- Then `dp[0] = true` (empty string is valid).
- For each `i` from `1` to `n`, we check all possible cut points `j < i`:
  - If `dp[j]` is `true` **and** `s[j..i)` is in the dictionary, then `dp[i] = true`.

We use a `HashSet` for `wordDict` to allow `O(1)` average-time lookups.  
We can also optimize by only looking back at most `max_word_length` characters.

This DP approach is chosen because:

- It checks all valid segmentations without brute-forcing every partition (which would be exponential).
- It reuses results of previous prefixes via `dp`, giving us polynomial time.

---

### 🪜 Steps

1. **Preprocess dictionary**
   - Convert `wordDict` into a `HashSet<String>` for fast membership checks.
   - Compute `max_len`, the length of the longest word in `wordDict` (optional optimization).

2. **Initialize DP array**
   - Let `n = s.len()`.
   - Create a boolean vector `dp` of size `n + 1`, all `false`.
   - Set `dp[0] = true` because the empty prefix is always segmentable.

3. **Fill DP using prefix checks**
   - For each `i` from `1` to `n`:
     - Let `start = max(0, i - max_len)` to avoid checking very long substrings unnecessarily.
     - For each `j` from `start` to `i - 1`:
       - If `dp[j] == true` **and** `s[j..i)` is in `wordDict`, then:
         - Set `dp[i] = true`.
         - Break the inner loop (no need to find more cuts for this `i`).

4. **Return the result**
   - The answer is `dp[n]`, indicating whether the entire string `s[0..n)` can be segmented.

---

## ✅ Constraints

From the LeetCode version of **Word Break**:

- `1 <= s.length <= 300`
- `1 <= wordDict.length <= 1000`
- `1 <= word.length <= 20`
- `s` and `wordDict[i]` consist of only lowercase English letters.
- All the strings in `wordDict` are **unique**.

---

## ⏱ Time and Space Complexity

Let:

- `n = len(s)`
- `L = max` word length in `wordDict`

| Metric            | Complexity           |
|-------------------|----------------------|
| Time Complexity   | O(n · L) to O(n²)\* |
| Space Complexity  | O(n)                |

\*In the worst case (no `max_len` optimization or when `L ≈ n`), the time complexity becomes `O(n²)`. With the `max_len` optimization, the inner loop is bounded by `L`, giving `O(n · L)`.

---
