# 🧩 LeetCode Problem: Binary Tree Inorder Traversal

- **Problem Link**: [Binary Tree Inorder Traversal – LeetCode](https://leetcode.com/problems/binary-tree-inorder-traversal/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/binary-tree-inorder-traversal/solutions/)

---

## 🧠 Algorithm Explanation

The inorder traversal of a binary tree visits nodes in the order:  
**Left → Root → Right**

We can solve this using either **recursion** or an **iterative approach with a stack**:

- **Recursive solution** is simple: visit the left child, then the current node, then the right child.
- **Iterative solution** simulates recursion using a stack: push nodes while going left, then pop, process the node, and move right.

Both ensure nodes are visited exactly once in the correct order.

---

### 🪜 Steps

1. Initialize an empty stack and result list.
2. Set a pointer to the root node.
3. While the pointer is not `None` or the stack is not empty:
   - Go as far left as possible, pushing nodes onto the stack.
   - Pop from the stack, add its value to the result.
   - Move to the right child.
4. Return the result list.

---

## ✅ Constraints

- `0 ≤ Number of nodes ≤ 100`
- Node values are integers.
- Tree may be empty.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)** – each node is visited once |
| Space Complexity  | **O(n)** – recursion stack or explicit stack |

---
