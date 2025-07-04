# 🧩 LeetCode Problem: Reverse Nodes in k-Group

- **Problem Link**: [Reverse Nodes in k-Group – LeetCode](https://leetcode.com/problems/reverse-nodes-in-k-group/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/reverse-nodes-in-k-group/solutions/)

---

## 🧠 Algorithm Explanation

We are given the head of a linked list and an integer \( k \).
We reverse the nodes of the list \( k \) at a time and return the modified list.
If the number of nodes left at the end is less than \( k \), we leave them as they are.

We use an **iterative in-place reversal** because:
- It runs in \( O(N) \) time (optimal).
- It uses \( O(1) \) extra space (optimal).
- It modifies only the node pointers, not their values.

---

### 🪜 Steps

1. **Initialize a dummy node** pointing to the head. This helps simplify pointer operations at the head of the list.
2. **Iterate through the list in groups of \( k \)**:
   - Use a pointer to find the \( k^{th} \) node ahead of the current position.
   - If fewer than \( k \) nodes remain, stop.
3. **Reverse the \( k \) nodes**:
   - Use three pointers (`prev`, `curr`, `next`) to reverse the links of \( k \) nodes in-place.
4. **Reconnect the reversed group**:
   - Connect the previous group's end to the start of the reversed group.
   - Move the `group_prev` pointer to the end of the newly reversed group.
5. Continue until you reach the end of the list.

---

## ✅ Constraints

- \( 1 \leq k \leq \text{length of linked list} \)
- The list contains at least one node.
- You **cannot change the values of the nodes**, only the pointers.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| **Time Complexity**   | \( O(N) \) — each node is visited and rewired exactly once |
| **Space Complexity**  | \( O(1) \) — only a few pointers are used, no extra data structures |

---
