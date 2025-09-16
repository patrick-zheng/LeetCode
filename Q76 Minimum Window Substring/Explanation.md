# 🧩 LeetCode Problem: Minimum Window Substring

- **Problem Link**: [Minimum Window Substring – LeetCode](https://leetcode.com/problems/minimum-window-substring/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/minimum-window-substring/solutions/)

---

## 🧠 Algorithm Explanation

We use a **sliding window** approach with frequency counts.

- Maintain a window `[l, r]` over `s` and track how many required characters from `t` are satisfied (including duplicates).
- Expand the right pointer `r` to include characters until the window is valid.
- Shrink from the left pointer `l` to find the shortest valid window, updating the best answer each time.
- This ensures the smallest substring of `s` that contains all characters of `t`.

---

### 🪜 Steps

1. **Count needs**: Build a frequency map of `t` and track the number of distinct required characters.
2. **Expand window**: Move `r` across `s`, updating the window frequency and marking satisfied characters.
3. **Shrink greedily**: When all required characters are present, move `l` forward to minimize the window while still valid.
4. **Record best**: Track the minimum window found. Return `""` if no valid window exists.

---

## ✅ Constraints

- Strings `s` and `t` with lengths `m` and `n`  
- `1 ≤ n ≤ m ≤ 10^5`  
- Characters may repeat; duplicates in `t` must be matched in the window.  
- The answer is guaranteed to be **unique**.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity  |
|-------------------|-------------|
| Time Complexity   | O(m + n)    |
| Space Complexity  | O(k)        |

Where `m = len(s)`, `n = len(t)`, and `k` is the number of distinct characters (bounded by charset size, e.g., ≤128 for ASCII).
