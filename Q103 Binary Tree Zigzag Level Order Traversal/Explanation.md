# 🧩 LeetCode Problem: Binary Tree Zigzag Level Order Traversal

- **Problem Link**: [Binary Tree Zigzag Level Order Traversal – LeetCode](https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/solutions/)

---

## 🧠 Algorithm Explanation

We perform a standard **BFS (level-order traversal)** with a queue. For each level, we gather node values in a temporary list. We maintain a boolean `left_to_right` that toggles after every level:

- When `True`, append values as encountered.
- When `False`, reverse the level list (or write into it from right to left) before pushing to the result.

Your provided code uses the simple **reverse-on-odd-levels** strategy, which is clean and `O(N)`.

> Optional optimization (not required): Instead of `reverse()`, preallocate a `level` array of size `n` and write `level[i] = val` when `left_to_right` else `level[n-1-i] = val`, avoiding the reversal.

---

### 🪜 Steps

1. **Initialize**  
   - If `root` is `None`, return `[]`.  
   - Create a queue `q` with `root`.  
   - Set `left_to_right = True`.  
   - Result list `res = []`.

2. **Process levels**  
   - While `q` is not empty, get the current level size `n = len(q)`.  
   - Pop `n` nodes, pushing their children into `q`.  
   - Collect their values in `level`.

3. **Adjust order & store**  
   - If `left_to_right` is `False`, reverse `level`.  
   - Append `level` to `res`.  
   - Toggle `left_to_right`.

4. **Return**  
   - After the loop, return `res`.

---

## ✅ Constraints

- Tree node count `N` in `[0, 2000]` (typical LeetCode constraint range).
- Node values can be any integers.
- Works for skewed trees, complete trees, and empty trees.

### Edge Cases

- `root = None` → `[]`
- Single node → `[[val]]`
- Skewed left/right trees → Alternation still respected

---

## ⏱ Time and Space Complexity

| Metric            | Complexity      |
|-------------------|-----------------|
| Time Complexity   | `O(N)`          |
| Space Complexity  | `O(W)` for the queue, where `W` is the tree’s max width (worst-case `O(N)`) |

---
