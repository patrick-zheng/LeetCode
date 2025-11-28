# 🧩 LeetCode Problem: Surrounded Regions

- **Problem Link**: [Surrounded Regions – LeetCode](https://leetcode.com/problems/surrounded-regions/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/surrounded-regions/solutions/)

---

## 🧠 Algorithm Explanation

We want to **capture** all `'O'` regions that are **fully surrounded** by `'X'`.

Key idea:

- Any `'O'` on the **border** or **connected to a border `'O'`** (4-directionally) can **never** be captured.
- So instead of trying to find “surrounded regions” directly, we:
  - Mark all **border-connected `'O'`s** as **safe**.
  - Then flip all remaining `'O'`s to `'X'`.

We do this with a **flood fill (BFS/DFS)** starting from all `'O'`s on the border and mark them with a temporary character (like `'E'` for “escaped”). In the end:

- `'O'` → `'X'` (captured region)
- `'E'` → `'O'` (safe / border-connected region)

This works well and runs in linear time in the size of the board.

---

### 🪜 Steps

1. **Check for empty board**  
   If `board` is empty or has no columns, return immediately.

2. **Traverse all border cells**  
   For every cell on:
   - first row (row `0`)
   - last row  (row `m-1`)
   - first column (col `0`)
   - last column (col `n-1`)

   If the cell is `'O'`, run BFS/DFS from there:
   - Change `'O'` to `'E'` (escaped).
   - Visit its 4 neighbors (up, down, left, right); for each `'O'`, repeat and mark as `'E'`.

3. **Flip captured and restore safe**  
   After marking all border-connected `'O'`s as `'E'`:
   - Scan the whole board:
     - If `board[i][j] == 'O'`: it’s fully surrounded → set to `'X'`.
     - If `board[i][j] == 'E'`: it’s safe → set back to `'O'`.

---

## ✅ Constraints

- `1 <= m, n <= 200`
- `board[i][j] ∈ {'X', 'O'}`
- Connectivity is **4-directional** (up, down, left, right).
- Must modify `board` **in-place** and not return anything.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity  |
|-------------------|------------|
| Time Complexity   | `O(m · n)` |
| Space Complexity  | `O(m · n)` in worst case (queue/stack for BFS/DFS) |

---
