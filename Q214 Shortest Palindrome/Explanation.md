# 🧩 LeetCode Problem: Shortest Palindrome (214)

- **Problem Link**: [Shortest Palindrome – LeetCode](https://leetcode.com/problems/shortest-palindrome/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/shortest-palindrome/solutions/)

---

## 🧠 Algorithm Explanation

We may only prepend characters, so any valid answer has the form `X + s` and must be a palindrome. The first `|s|` characters of a palindrome mirror the last `|s|` characters; the tail is fixed as `s`, so those first `|s|` characters must equal `reverse(s)` in order. The part of `s` that does not need “new” mirror characters is exactly a **palindromic prefix**. If the **longest** palindromic prefix has length `L`, the shortest choice is to prepend `reverse(s[L:])`, giving `reverse(s[L:]) + s`.

That reduces the problem to computing **`L` in linear time**. A prefix `s[0:L]` is a palindrome iff it matches the last `L` characters of `reverse(s)`. Build `T = s + '#' + reverse(s)` (with `#` not in `s`, which holds for this problem’s lowercase letters). Run the KMP **prefix function (LPS)** on `T`: the **last** LPS value is the length of the longest prefix of `T` that is also a suffix of `T`, which equals **`L`**. The answer is the first `|s| - L` characters of `reverse(s)` followed by `s`.

**Why this algorithm:** Brute force checking every prefix for palindrome is **O(n²)**. KMP LPS on a string of length **O(n)** yields **O(n)** time and **O(n)** space, which fits the constraints.

---

### 🪜 Steps

1. **Reduce to longest palindromic prefix**: Show that the shortest answer is `reverse(s[L:]) + s` where `L` is the length of the longest palindrome prefix of `s` (empty `s` returns immediately).

2. **Compute `L` with KMP**: Form `T = s + '#' + reverse(s)` and fill the LPS array by scanning `T` once, using the standard fallback on mismatch.

3. **Build the result**: Let `L = LPS[|T| - 1]` and return `reverse(s)[0 : |s| - L] + s`.

---

## ✅ Constraints

- `0 <= s.length <= 5 * 10^4`
- `s` consists of lowercase English letters only

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |

---
