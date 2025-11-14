# 🧩 LeetCode: Triangle Minimum Path Sum

> Given a triangle array, return the **minimum path sum from top to bottom**.  
> From index `i` in a row, you may go to index `i` or `i + 1` in the next row.

---

## 🧠 Algorithm Explanation (Bottom-Up DP)

We can solve this using **dynamic programming from the bottom row up**:

- Any path from the top must eventually pass through the bottom row.
- If we know, for each cell in a row, the **minimum path sum from that cell to the bottom**,  
  then the row above can use that information to compute its own best sums.

So we:

1. Start from the **last row**: the minimum path sum from those elements to the bottom is just their own values.
2. Move upwards row by row.
3. At each step (row r, column c), we update dp like this:
    $$
    dp[c] = triangle[r][c] + min(dp[c], dp[c + 1])
    $$
    Where:

    - dp[c] is the minimum path sum from position (r, c) down to the bottom.
    - dp[c] and dp[c + 1] (on the right-hand side) are from the row below.

   where `dp[c]` and `dp[c+1]` are the minimum sums from the two reachable positions in the row below.

4. At the end, `dp[0]` will contain the **minimum path sum from the top to the bottom**.

We can reuse a **single array `dp`** (same length as the last row), so space is O(n).

---

### 🪜 Steps

1. **Initialize** `dp` as a copy of the **last row** of the triangle.
2. **Iterate upwards** from the second-last row to the top:
   - For each index `c` in row `r`:
     - Update `dp[c] = triangle[r][c] + min(dp[c], dp[c+1])`.
3. After finishing the top row, **return `dp[0]`**.

---

## ✅ Constraints (Typical LeetCode-Style)

- `1 <= triangle.length <= 200`
- `triangle[i].length == i + 1`
- Triangle values are integers (can be negative or positive).

---

## ⏱ Time and Space Complexity

| Metric           | Complexity |
|------------------|------------|
| Time Complexity  | O(n²)      |
| Space Complexity | O(n)       |

Where `n` is the number of rows in the triangle (total elements is ~ n²/2).

---
