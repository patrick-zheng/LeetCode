# 🧩 LeetCode Problem: Decode Ways

- **Problem Link**: [Decode Ways – LeetCode](https://leetcode.com/problems/decode-ways/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/decode-ways/solutions/)

---

## 🧠 Algorithm Explanation

The problem is about finding how many ways we can decode a string of digits into letters using the mapping:

- "1" → A, "2" → B, …, "26" → Z.

Key observations:

- A single non-zero digit (`'1'..'9'`) can always be decoded as one letter.
- Two digits form a valid letter only if they lie between `"10"` and `"26"`.
- A `'0'` is invalid alone; it must appear as part of `"10"` or `"20"`.

This naturally leads to a **dynamic programming** approach:

- Let `dp[i]` represent the number of ways to decode the string up to index `i`.
- For each position:
  - If `s[i] != '0'`, add `dp[i-1]` (single digit).
  - If `10 <= s[i-1..i] <= 26`, add `dp[i-2]` (two-digit).

We can optimize to O(1) space by storing only the last two states (`prev1`, `prev2`).

---

### 🪜 Steps

1. **Initialization**:  
   - If the string starts with `'0'`, return 0 (invalid).  
   - Set `prev2 = 1` (empty prefix has 1 way), `prev1 = 1`.

2. **Iterate through string**:  
   - For each index `i`, compute `curr` as the number of ways to decode up to that index:
     - If `s[i] != '0'`, add `prev1`.
     - If two-digit number formed with `s[i-1]` is between 10 and 26, add `prev2`.

3. **Update state**:  
   - Move `prev2 = prev1`, `prev1 = curr`.

4. **Final Answer**:  
   - Return `prev1`.

---

## ✅ Constraints

- `1 <= s.length <= 100`
- `s` contains only digits `'0'–'9'`
- Result fits in a 32-bit integer

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n), where n = length of string |
| Space Complexity  | O(1), using only two variables for DP |

---
