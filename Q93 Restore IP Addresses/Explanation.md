# 🧩 LeetCode Problem: Decode Ways

- **Problem Link**: [Decode Ways – LeetCode](https://leetcode.com/problems/decode-ways/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/decode-ways/solutions/)

---

## 🧠 Algorithm Explanation

We are asked to count how many ways we can decode a string of digits into letters (`"1" -> A, "2" -> B, ... "26" -> Z`).  
This is a **dynamic programming** problem because each character can contribute to multiple decoding paths, depending on whether it is taken alone or paired with the previous character.

We use the following idea:

- A digit `'0'` cannot stand alone. It must pair with `'1'` or `'2'` to form `"10"` or `"20"`.
- At each index, we check:
  - If the current single digit is valid (`1–9`), add ways from previous step.
  - If the two-digit number (previous + current) is valid (`10–26`), add ways from two steps back.
- Build this iteratively with DP or recursively with memoization.

---

### 🪜 Steps

1. **Initialization**:  
   - `dp[0] = 1` (empty string has one way to decode).  
   - `dp[1] = 1` if first digit is not `'0'`, otherwise `0`.

2. **Iterate through the string**:  
   - For each position `i`, check the single digit `s[i-1]`.  
   - Also check the two-digit substring `s[i-2:i]`.

3. **Update DP**:  
   - If valid single digit → add `dp[i-1]`.  
   - If valid two digits → add `dp[i-2]`.

4. **Answer**:  
   - The result is `dp[n]` where `n = len(s)`.

---

## ✅ Constraints

- Input string length: `1 ≤ n ≤ 100`.
- Only contains digits `'0'–'9'`.
- Must form valid decodings (`"0"` is invalid unless part of `"10"` or `"20"`).

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n) or O(1) with optimized DP |

---
