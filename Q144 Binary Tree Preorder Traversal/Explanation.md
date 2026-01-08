# 🧩 LeetCode Problem: Binary Tree Preorder Traversal

- **Problem Link**: <https://leetcode.com/problems/binary-tree-preorder-traversal/>
- **Solution Link**: <https://leetcode.com/problems/binary-tree-preorder-traversal/solutions/>

---

## 🧠 Algorithm Explanation

This problem asks us to return the **preorder traversal** of a binary tree.

**Preorder traversal order**:

1. Visit the **root**
2. Traverse the **left subtree**
3. Traverse the **right subtree**

A **recursive Depth-First Search (DFS)** approach is ideal here because:

- Binary trees are naturally recursive structures.
- Preorder traversal directly maps to a simple recursive pattern.
- The solution is clean, readable, and efficient.

---

### 🪜 Steps

1. **Initialize a result list**
   - This list will store the traversal order.

2. **Define a recursive helper function**
   - If the current node is `None`, return.
   - Otherwise:
     - Add the node’s value to the result list.
     - Recursively traverse the left child.
     - Recursively traverse the right child.

3. **Call the helper function starting from the root**
   - Return the final result list.

---

## ✅ Constraints

- The number of nodes in the tree is in the range `[0, 100]`
- Node values are integers
- The tree may be empty (`root = None`)

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)**   |
| Space Complexity  | **O(n)**   |
