# 🧩 LeetCode Problem: Interleaving String

- **Problem Link**: [Interleaving String – LeetCode](https://leetcode.com/problems/interleaving-string/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/interleaving-string/solutions/)

---

## 🧠 Algorithm Explanation

We need to determine if `s3` can be formed by interleaving characters from `s1` and `s2`, while preserving their relative order.  
We use **Dynamic Programming (DP)** since each prefix of `s3` depends on whether we can reach it by taking the next character from `s1` or from `s2`.

---

### 🪜 Steps

1. **Step 1**: If `len(s1) + len(s2) != len(s3)`, immediately return `False`.
2. **Step 2**: Define a DP array `dp[j]` where `dp[j]` means `s3[:i+j]` can be formed using `s1[:i]` and `s2[:j]`.
3. **Step 3**: Initialize the first row using only characters from `s2`.
4. **Step 4**: For each character in `s1`, update the DP array:
   - `dp[j] = (dp[j] and s1[i-1] == s3[i+j-1]) or (dp[j-1] and s2[j-1] == s3[i+j-1])`
5. **Step 5**: Return `dp[m]` as the final result.

---

## ✅ Constraints

- `0 <= len(s1), len(s2) <= 100`
- `0 <= len(s3) <= 200`
- `len(s1) + len(s2) == len(s3)`
- Strings consist only of lowercase English letters.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n × m)   |
| Space Complexity  | O(min(n, m)) |

---
