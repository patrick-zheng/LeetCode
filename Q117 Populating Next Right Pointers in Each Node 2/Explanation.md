# 🧩 LeetCode Problem: Populating Next Right Pointers in Each Node II

- **Problem Link**: [Populating Next Right Pointers in Each Node II – LeetCode](https://leetcode.com/problems/populating-next-right-pointers-in-each-node-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/populating-next-right-pointers-in-each-node-ii/solutions/)

---

## 🧠 Algorithm Explanation

We traverse the tree **level by level** using already-built `next` pointers.  
For each level, we stitch together the **next level** with a **dummy head** and a moving **tail**:

- Scan the current level via `curr = curr.next`.
- When a node has `left`/`right`, append them to `tail.next` and advance `tail`.
- After finishing a level, jump to `dummy.next` to start the next level.

This uses **O(1) extra space** (no queue) and works for **any** binary tree.

---

### 🪜 Steps

1. **Init**: `curr = root`.
2. **Prepare next level**: Create `dummy = Node(0)`, `tail = dummy`.
3. **Scan current level**:
   - If `curr.left`: `tail.next = curr.left`, move `tail`.
   - If `curr.right`: `tail.next = curr.right`, move `tail`.
   - Move along: `curr = curr.next`.
4. **Descend**: `curr = dummy.next` (first node of the next level).
5. **Repeat** until `curr` is `None`.

---

## ✅ Constraints

- Tree can be any shape (not necessarily perfect).
- If no right neighbor exists, the node’s `next` must be `None`.
- Aim for **O(1) extra space** (beyond a few pointers).

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(N)       |
| Space Complexity  | O(1)       |

---
