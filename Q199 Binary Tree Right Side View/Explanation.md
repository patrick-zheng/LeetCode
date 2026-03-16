# 🧩 LeetCode Problem: Binary Tree Right Side View

- **Problem Link**: [Binary Tree Right Side View – LeetCode](https://leetcode.com/problems/binary-tree-right-side-view/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/binary-tree-right-side-view/solutions/)

---

## 🧠 Algorithm Explanation

The goal is to return the node values visible when looking at the binary tree from the right side, from top to bottom.

An optimal approach is to use **Depth-First Search (DFS)** while always visiting the **right child before the left child**. This works because the first node we encounter at each depth is the one that would be visible from the right side.

We keep a result array. During traversal, if we reach a depth that has not been seen before, we add that node’s value to the result.

This is optimal because every node is visited only once.

---

### 🪜 Steps

1. **Step 1**: Start a DFS traversal from the root, and pass the current depth as a parameter.

2. **Step 2**: At each node, if this is the first time reaching this depth, add the node’s value to the result.

3. **Step 3**: Recurse on the right child first, then the left child, so the rightmost visible node is processed first at every level.

---

## ✅ Constraints

- The number of nodes in the tree is in the range `[0, 100]`.
- `-100 <= Node.val <= 100`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(h)       |

---
