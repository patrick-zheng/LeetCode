# 🧩 LeetCode Problem: Reverse Linked List II

- **Problem Link**: [Reverse Linked List II – LeetCode](https://leetcode.com/problems/reverse-linked-list-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/reverse-linked-list-ii/solutions/)

---

## 🧠 Algorithm Explanation

We need to reverse a portion of a singly linked list between positions `left` and `right`.  
Instead of copying values or creating a new list, we can do it in-place by rearranging pointers.

The idea is:

1. Use a dummy node before head to handle edge cases cleanly.
2. Move a pointer (`prev`) to just before the sublist that will be reversed.
3. Iteratively reverse the sublist by adjusting `next` pointers, keeping track of three nodes: `prev`, `curr`, and `nxt`.
4. Reconnect the reversed part back to the rest of the list.

This ensures only **O(1) extra space** and **O(n) time**.

---

### 🪜 Steps

1. **Initialize** a dummy node before head and move `prev` to the node before `left`.  
2. **Reverse** nodes between `left` and `right` by moving one node at a time to the front of the sublist.  
3. **Reconnect** the reversed sublist to the main list and return the new head.

---

## ✅ Constraints

- The list has between 1 and 500 nodes.  
- `1 ≤ left ≤ right ≤ length of list`.  

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
