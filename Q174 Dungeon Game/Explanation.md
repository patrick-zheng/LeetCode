# 🧩 LeetCode Problem: Dungeon Game

- **Problem Link**: [Dungeon Game – LeetCode](https://leetcode.com/problems/dungeon-game/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/dungeon-game/solutions/)

---

## 🧠 Algorithm Explanation

This problem is solved using **Dynamic Programming (DP)** by working **backwards from the destination** instead of simulating the knight’s health forward.  
For each cell, compute the **minimum health needed upon entering** so the knight can still reach the princess while keeping health **≥ 1** at all times. Since moves are only **right** or **down**, each cell depends only on those two neighbors.

**DP definition:** `dp[i][j]` = minimum HP required to **enter** cell `(i, j)` and still survive to the end.  
**Transition:**  
`dp[i][j] = max(1, min(dp[i+1][j], dp[i][j+1]) - dungeon[i][j])`  
**Base case (princess cell):**  
`dp[m-1][n-1] = max(1, 1 - dungeon[m-1][n-1])`  
Space can be optimized to **O(n)** using a 1D rolling array.

---

### 🪜 Steps

1. **Create DP meaning**: `dp[i][j]` is the minimum HP needed to enter `(i, j)` and eventually reach the princess alive.  
2. **Initialize base** at bottom-right: need at least `max(1, 1 - dungeon[m-1][n-1])`.  
3. **Fill backwards** from bottom-right to top-left using:  
   `dp[i][j] = max(1, min(down, right) - dungeon[i][j])`.  
4. **Answer** is `dp[0][0]` (minimum initial HP at the start).

---

## ✅ Constraints

- `1 ≤ m, n ≤ 200`
- `-1000 ≤ dungeon[i][j] ≤ 1000`
- Health must always be **≥ 1**
- Moves allowed: **right** or **down**

---

## ⏱ Time and Space Complexity

| Metric            |  Complexity  |
|-------------------|--------------|
| Time Complexity   | **O(m · n)** |
| Space Complexity  |   **O(n)**   |

---
