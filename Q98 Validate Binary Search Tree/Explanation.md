# 🧩 LeetCode Problem: Validate Binary Search Tree (BST)

- **Problem Link**: [Validate Binary Search Tree – LeetCode](https://leetcode.com/problems/validate-binary-search-tree/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/validate-binary-search-tree/solutions/)

---

## 🧠 Algorithm Explanation

Use DFS with value bounds. For each node `v`, ensure `low < v < high`.  
Recurse left with `(low, v)` and right with `(v, high)`.  
This enforces the strict BST rule across the whole subtree.

*Alternative:* Inorder traversal must be strictly increasing.

---

### 🪜 Steps

1. Start at the root with bounds `(-∞, +∞)`.
2. If the node is `None`, it’s valid.
3. Check `low < node.val < high`.
4. Recurse:
   - Left: `(low, node.val)`
   - Right: `(node.val, high)`
5. Return `left_ok and right_ok`.

---

## ✅ Constraints

- Nodes: up to `10^4`
- Values: `[-2^31, 2^31-1]`
- Tree can be skewed

---

## ⏱ Time and Space Complexity

| Metric            | Complexity        |
|-------------------|-------------------|
| Time Complexity   | `O(n)`            |
| Space Complexity  | `O(h)` (worst `O(n)`) |

---
