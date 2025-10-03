# 🧩 LeetCode Problem: Scramble String

- **Problem Link**: [Scramble String – LeetCode](https://leetcode.com/problems/scramble-string/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/scramble-string/solutions/)

---

## 🧠 Algorithm Explanation

The **Scramble String** problem asks whether one string can be transformed into another by recursively splitting and optionally swapping substrings.  
This recursive structure makes it a perfect candidate for **dynamic programming with memoization**.

Key ideas:

1. If two substrings are equal, they are trivially scrambles.
2. If two substrings don’t have the same multiset of characters, they cannot be scrambles.
3. Otherwise, try every possible split position `k`. For each, check:
   - **No swap case**: left of `s1` matches left of `s2` and right matches right.
   - **Swap case**: left of `s1` matches right of `s2` and right matches left.

Using memoization avoids recomputation of overlapping subproblems.  
We also prune recursion early with fast character frequency checks.

---

### 🪜 Steps

1. **Base checks**  
   - If lengths differ → return false.  
   - If strings are identical → return true.  
   - If character counts differ → return false.

2. **Recursive split**  
   - For each split index `k` (from 1 to n-1), check both:
     - No-swap match.
     - Swap match.

3. **Memoize results**  
   - Store results of `(i, j, length)` states to avoid recomputation.

---

## ✅ Constraints

- `1 <= s1.length <= 30`
- `s1.length == s2.length`
- `s1` and `s2` consist of lowercase English letters.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n³)** worst case (each substring split tested, with pruning) |
| Space Complexity  | **O(n³)** for memoization |

---
