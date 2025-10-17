# 🧩 LeetCode Problem: Same Tree (100)

- **Problem Link**: [Same Tree – LeetCode](https://leetcode.com/problems/same-tree/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/same-tree/solutions/)

---

## 🧠 Algorithm Explanation

Two binary trees are the **same** if they are structurally identical and every corresponding node has the same value.  
We compare the roots:

- If both are `None` → same.
- If exactly one is `None` or values differ → not the same.
- Otherwise, they must have the same left subtrees **and** the same right subtrees.  
This naturally leads to a simple DFS recursion.

---

### 🪜 Steps

1. **Base cases**  
   - If `p is None` and `q is None` → return `True`.  
   - If exactly one is `None` → return `False`.
2. **Value check**  
   - If `p.val != q.val` → return `False`.
3. **Recurse**  
   - Return `isSameTree(p.left, q.left) and isSameTree(p.right, q.right)`.

---

## ✅ Constraints

- Number of nodes in both trees: `0 ≤ nodes ≤ 100`
- Node values: `-10^4 ≤ val ≤ 10^4`
- Trees can be empty.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity     |
|-------------------|----------------|
| Time Complexity   | O(n)           |
| Space Complexity  | O(h) recursion |

- `n` = total nodes visited (min of sizes if early mismatch).  
- `h` = tree height (O(log n) for balanced, O(n) worst-case skewed).

---
