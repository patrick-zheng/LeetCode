# 🧩 LeetCode Problem: Binary Tree Postorder Traversal

- **Problem Link**: [Binary Tree Postorder Traversal – LeetCode](https://leetcode.com/problems/binary-tree-postorder-traversal/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/binary-tree-postorder-traversal/solutions/)

---

## 🧠 Algorithm Explanation

This problem asks us to return the **postorder traversal** of a binary tree.

**Postorder traversal order**:

1. Left subtree
2. Right subtree
3. Root node

A **recursive Depth-First Search (DFS)** approach is ideal here because:

- Trees are naturally recursive data structures
- Postorder traversal directly matches the recursive pattern
- The solution is clean, readable, and easy to reason about

---

### 🪜 Steps

1. **Base Case**  
   - If the current node is `None`, return immediately.

2. **Traverse Left Subtree**  
   - Recursively call the function on `node.left`.

3. **Traverse Right Subtree**  
   - Recursively call the function on `node.right`.

4. **Process Current Node**  
   - Append `node.val` to the result list **after** visiting both subtrees.

---

## ✅ Constraints

- The number of nodes in the tree is in the range `[0, 100]`
- Node values are integers
- The tree can be empty (`root = None`)

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)**   |
| Space Complexity  | **O(n)**   |

**Explanation**:

- Each node is visited exactly once → **O(n)** time
- Recursive call stack can grow up to the height of the tree  
  - Worst case (skewed tree): **O(n)**
  - Best case (balanced tree): **O(log n)**  
  - LeetCode convention counts this as **O(n)** auxiliary space

---
