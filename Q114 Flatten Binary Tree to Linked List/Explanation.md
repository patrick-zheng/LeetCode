# 🧩 LeetCode Problem: Flatten Binary Tree to Linked List

- **Problem Link**: [Flatten Binary Tree to Linked List – LeetCode](https://leetcode.com/problems/flatten-binary-tree-to-linked-list/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/flatten-binary-tree-to-linked-list/solutions/)

---

## 🧠 Algorithm Explanation

We want to transform a binary tree **in-place** into a singly linked list that follows **preorder traversal** (root → left → right).  
A clean way to do this is to **simulate preorder with a stack** and **rewire pointers as we go**:

- Keep a `prev` pointer to the previously visited node in preorder.
- For each popped node `curr`, set `prev.left = None` and `prev.right = curr`.
- Push `curr.right` first, then `curr.left` so the left subtree is processed before the right (LIFO).

This preserves preorder order while producing a right-skewed list.

---

### 🪜 Steps

1. **Init**: If `root` is `None`, return. Create a stack with `[root]` and set `prev = None`.
2. **Traverse**:
   - Pop `curr` from the stack.
   - If `prev` exists, set `prev.left = None` and `prev.right = curr` to link the list.
3. **Push children**:
   - If `curr.right`, push it.
   - If `curr.left`, push it (so it’s processed next).
4. **Advance**: Set `prev = curr` and continue until the stack is empty.
5. **Result**: Tree is flattened in place; `root` now heads a right-only chain in preorder.

---

## ✅ Constraints

- Binary tree node count `n` fits in typical LeetCode limits (e.g., up to `10^4`).
- Values can be any integers; structure matters, not values.
- Must **not** allocate a new list structure; modify the tree **in place**.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity   |
|-------------------|--------------|
| Time Complexity   | **O(n)**     |
| Space Complexity  | **O(h)** stack (worst **O(n)** for skewed trees) |

> **Note**: There’s also a **Morris-style** constant-extra-space solution (O(1) extra) by finding each node’s preorder predecessor and splicing right pointers, but the stack method is simpler and accepted.

---
