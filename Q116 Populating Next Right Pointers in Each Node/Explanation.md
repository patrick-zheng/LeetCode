# 🧩 LeetCode Problem: Populating Next Right Pointers in Each Node (Perfect Binary Tree)

- **Problem Link**: [LeetCode 116 – Populating Next Right Pointers in Each Node](https://leetcode.com/problems/populating-next-right-pointers-in-each-node/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/populating-next-right-pointers-in-each-node/solutions/)

---

## 🧠 Algorithm Explanation

Because the tree is **perfect** (every node has two children and all leaves are on the same level), each node’s `left` child’s next is its `right` sibling, and each node’s `right` child’s next is the **left child of the node’s right neighbor** (if that neighbor exists).  
We can connect level by level using already-populated `next` pointers from the previous level—this achieves **O(1)** extra space.

---

### 🪜 Steps

1. **Initialize**: If `root` is `null`, return `null`. Set `leftmost = root` to start from the top level.
2. **Process a level** (while `leftmost->left` exists):
   - Set `cur = leftmost` to traverse the current level using `next` pointers.
   - For each `cur`:
     - Connect siblings: `cur->left->next = cur->right`.
     - Connect across parents: if `cur->next` exists, set `cur->right->next = cur->next->left`.
     - Move right: `cur = cur->next`.
3. **Go down one level**: After finishing the level, move `leftmost = leftmost->left`.
4. **Stop condition**: When `leftmost` has no left child (we’re at the leaves), we’re done.

---

## ✅ Constraints

- The binary tree is **perfect**: every internal node has exactly two children; all leaves are at the same depth.
- Node count `n` fits in typical memory limits.
- Values in nodes are irrelevant for pointer wiring.
- Desired extra space is **O(1)** (not counting recursion stack).

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
