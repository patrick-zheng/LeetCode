# 🧩 LeetCode Problem: Reverse Linked List

- **Problem Link**: [206. Reverse Linked List – LeetCode](https://leetcode.com/problems/reverse-linked-list/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/reverse-linked-list/solutions/)

---

## 🧠 Algorithm Explanation

The optimal solution is to reverse the linked list **in-place** using an **iterative pointer approach**.

We keep track of:

- `prev`: the previous node
- `curr`: the current node
- `next`: the next node before changing links

For each node, we reverse its `next` pointer so it points to the previous node instead of the next one. Then we move all pointers forward until the list is fully reversed.

This is optimal because:

- each node is visited exactly once
- no extra data structure is needed
- reversal is done directly on the existing list

---

### 🪜 Steps

1. Initialize `prev = None` (or `nullptr` / `None` equivalent) and `curr = head`.
2. While `curr` is not null:
   - store `curr.next` in a temporary variable
   - point `curr.next` to `prev`
   - move `prev` to `curr`
   - move `curr` to the saved next node
3. Return `prev`, which becomes the new head of the reversed list.

---

## ✅ Constraints

- The number of nodes in the list is in the range `[0, 5000]`
- `-5000 <= Node.val <= 5000`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
