# 🧩 LeetCode Problem: Convert Sorted List to Binary Search Tree

- **Problem Link**: [Convert Sorted List to Binary Search Tree – LeetCode](https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/solutions/)

---

## 🧠 Algorithm Explanation

A height-balanced BST’s **inorder traversal is sorted**. Since the linked list is already sorted, we can **simulate an inorder build** in one pass:

1. Count the list length `n`.
2. Recursively build a subtree of size `k`:
   - Build left subtree of size `k//2`.
   - Use the **current** list node as root (advance pointer).
   - Build right subtree of size `k - 1 - k//2`.

This visits each node exactly once ⇒ **O(n)** time and **O(log n)** stack (balanced).

---

### 🪜 Steps

1. **Count length** `n` by walking the list once.  
2. **Define `build(k)`** → returns a balanced BST from the next `k` nodes.  
3. **Inorder simulation**:
   - `left = build(k//2)`
   - `root = TreeNode(curr.val)`; advance `curr = curr.next`
   - `right = build(k - 1 - k//2)`; attach to `root`
4. **Answer** is `build(n)`.

---

## ✅ Constraints

- Input: singly linked list, sorted non-decreasing.
- Output: height-balanced BST.
- Duplicates allowed; negatives allowed.
- Empty list ⇒ return `None`.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity      |
|-------------------|-----------------|
| Time Complexity   | **O(n)**        |
| Space Complexity  | **O(log n)** (recursion stack) |

---
