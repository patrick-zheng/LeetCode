# 🧩 LeetCode Problem: N-Queens

- **Problem Link**: [N-Queens – LeetCode](https://leetcode.com/problems/n-queens/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/n-queens/solutions/)

---

## 🧠 Algorithm Explanation

We use **backtracking** to systematically explore all possible ways to place `n` queens on an `n x n` chessboard such that no two queens attack each other.  
The algorithm maintains sets to track:

- Columns already occupied by a queen
- Main diagonals (`row - col`)
- Anti-diagonals (`row + col`)

Backtracking allows us to try placing a queen in each row, recursively proceed to the next row if the placement is valid, and undo the move if it leads to a dead end.

This method is chosen because:

- It systematically searches all configurations.
- It prunes invalid placements early, reducing unnecessary exploration.

---

### 🪜 Steps

1. **Initialize Board & State Trackers**  
   Create an empty board and three sets to track used columns, main diagonals, and anti-diagonals.

2. **Recursive Backtracking**  
   At each row, iterate over all columns:
   - Skip if column, main diagonal, or anti-diagonal is already occupied.
   - Otherwise, place a queen and mark the column/diagonals as used.
   - Recur to place the queen in the next row.

3. **Backtrack & Undo**  
   If a valid configuration is found (row == n), save it.
   Remove the queen and unmark column/diagonals to explore other possibilities.

---

## ✅ Constraints

- `1 <= n <= 9`
- Only valid configurations should be returned.
- Output format: List of boards, where each board is represented as a list of strings.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n!)      |
| Space Complexity  | O(n²)      |

**Explanation**:  

- Time complexity is roughly `O(n!)` because we try permutations of queen placements with pruning.  
- Space complexity is `O(n²)` for storing the board, plus `O(n)` for the tracking sets.

---
