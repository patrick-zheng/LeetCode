# 🧩 LeetCode Problem: Minimum Depth of Binary Tree

- **Problem Link**: [Minimum Depth of Binary Tree – LeetCode](https://leetcode.com/problems/minimum-depth-of-binary-tree/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/minimum-depth-of-binary-tree/solutions/)

---

## 🧠 Algorithm Explanation

We need the **length of the shortest path from the root to any leaf**. A leaf is a node with **no left and no right child**.

The most efficient way to get the *minimum* depth is to use **Breadth-First Search (BFS)** (level-order). BFS visits nodes level by level. The **first time** we see a leaf, we **immediately know** its depth is the minimum, so we can return right away. This avoids exploring deeper subtrees unnecessarily.

We don’t use a simple recursive `min(left, right)` without care because a node may have **only one child** — in that case we *must* continue down the existing child.

---

### 🪜 Steps

1. **Check empty tree**:  
   If `root` is `null`, the minimum depth is `0`.

2. **Initialize queue**:  
   Put `(root, 1)` in a queue, where `1` is the current depth.

3. **Level-order traversal**:  
   While the queue is not empty:
   - Pop the front node and its depth.
   - If this node is a **leaf** (no left, no right), **return** its depth — this is the minimum.
   - Otherwise, push its non-null children to the queue with depth `+1`.

---

## ✅ Constraints

- The number of nodes is in the range **[0, 10⁵]**.
- Node values do **not** affect the answer.
- Tree may be **skewed** (like a linked list), so the algorithm must handle deep trees safely.
- Must return **0** for an empty tree.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(N)** – in the worst case we visit all nodes |
| Space Complexity  | **O(W)** – where **W** is the maximum width of the tree (queue size) |

---
