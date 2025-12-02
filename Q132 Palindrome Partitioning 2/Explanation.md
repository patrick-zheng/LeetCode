# 🧩 LeetCode Problem: Palindrome Partitioning II

- **Problem Link**: [Palindrome Partitioning II – LeetCode](https://leetcode.com/problems/palindrome-partitioning-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/palindrome-partitioning-ii/solutions/)

---

## 🧠 Algorithm Explanation

We want to split the string `s` into substrings such that **each substring is a palindrome**, and we want the **minimum number of cuts**.

Brute force would try all possible ways to cut the string and check each partition, which is exponential.  
Instead, we use **Dynamic Programming** with two key ideas:

1. **Precompute all palindromic substrings** using a 2D table `pal[i][j]` that tells us if `s[i..j]` is a palindrome.
2. Use a **1D DP** array `dp[i]` where:
   - `dp[i]` = minimum cuts needed to partition `s[0..i]` into palindromes.

For each index `i`, we look back at all possible positions `j < i` such that `s[j+1..i]` is a palindrome.  
If `s[0..i]` itself is a palindrome, then `dp[i] = 0`.  
Otherwise:  
`dp[i] = min( dp[j] + 1 )` for all `j < i` with `s[j+1..i]` palindrome.

We precompute `pal[i][j]` in **O(n²)** and compute `dp` also in **O(n²)**, giving a total of **O(n²)**.

---

### 🪜 Steps

1. **Precompute palindrome table**  
   - Create a 2D boolean array `pal[n][n]`, where `pal[i][j]` is `true` if `s[i..j]` is a palindrome.
   - Fill it from the back:
     - `pal[i][j] = (s[i] == s[j]) && (j - i < 2 || pal[i+1][j-1])`.

2. **Initialize DP array**  
   - Create an array `dp[n]`.
   - `dp[i]` will store the minimum cuts for substring `s[0..i]`.
   - Initialize worst case as `dp[i] = i` (cut before every character).

3. **Compute minimum cuts using DP**  
   - For each `i` from `0` to `n-1`:
     - If `pal[0][i]` is `true`, then `dp[i] = 0` (whole prefix is a palindrome).
     - Otherwise, try all `j` from `0` to `i-1`:
       - If `pal[j+1][i]` is `true`, then:
         - Update `dp[i] = min(dp[i], dp[j] + 1)`.
   - The final answer is `dp[n-1]`.

---

## ✅ Constraints

- `1 <= s.length <= 2000`
- `s[i]` is a lowercase English letter
- Need an algorithm faster than brute force (must be around **O(n²)**).

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n²)      |
| Space Complexity  | O(n²)      |

---
