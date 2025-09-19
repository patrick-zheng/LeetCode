# 🧩 LeetCode Problem: Word Search

- **Problem Link**: [Word Search – LeetCode](https://leetcode.com/problems/word-search/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/word-search/solutions/)

---

## 🧠 Algorithm Explanation

We solve this using **backtracking (DFS)**:

- Start DFS from any cell that matches the first letter of the word.  
- Move up, down, left, or right to continue matching letters.  
- Mark a cell as visited (temporarily change its value) to avoid reusing it in the same path.  
- Backtrack (restore the cell) after exploring.  
- Use **pruning optimizations**:  
  1. **Letter-count check** — if the board has fewer instances of any character than the word requires, return `False`.  
  2. **Rarer-end search** — reverse the word if its last character is rarer than the first to reduce branching.  

---

### 🪜 Steps

1. **Pre-checks**: Count letters in the board vs. word → prune impossible cases.  
2. **Optimize start**: Reverse the word if the last letter is rarer than the first.  
3. **DFS Search**:  
   - If the current cell does not match → backtrack.  
   - If last letter matches → success.  
   - Mark cell as visited and explore 4 neighbors.  
   - Restore cell before returning.  
4. **Return**: If any DFS path succeeds, return `True`; else return `False`.

---

## ✅ Constraints

- Grid size: `m x n`, with `1 ≤ m, n`.  
- Word length ≥ 1.  
- Moves allowed: up, down, left, right (no diagonals).  
- Each board cell may only be used once per search path.  

---

## ⏱ Time and Space Complexity

| Metric            | Complexity                   |
|-------------------|------------------------------|
| Time Complexity   | \(O(R \cdot C \cdot 4^{L})\) |
| Space Complexity  | \(O(L)\) recursion stack     |

- \(R, C\): grid dimensions.  
- \(L\): word length.  
- Extra space is constant beyond recursion because we use in-place marking.

---
