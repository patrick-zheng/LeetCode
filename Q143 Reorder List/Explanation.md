# 🧩 LeetCode Problem: Reorder List

- **Problem Link**: [Reorder List – LeetCode](https://leetcode.com/problems/reorder-list/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/reorder-list/solutions/)

---

## 🧠 Algorithm Explanation

The goal is to reorder a singly linked list **in-place** without modifying node values.  
The optimal approach avoids extra memory by manipulating pointers directly.

The algorithm works by:

- Splitting the list into two halves
- Reversing the second half
- Merging the two halves in alternating order

This strategy is optimal because it processes each node a constant number of times and uses no additional data structures.

---

### 🪜 Steps

1. **Find the middle of the list**  
   Use the fast and slow pointer technique. When the fast pointer reaches the end, the slow pointer is at the midpoint.

2. **Reverse the second half**  
   Reverse the linked list starting from the node after the midpoint. This allows easy access to nodes from the end of the original list.

3. **Merge the two halves alternately**  
   Interleave nodes from the first half and the reversed second half to achieve the required order:
   `L0 → Ln → L1 → Ln-1 → ...`

---

## ✅ Constraints

- The list must be reordered **in-place**
- Node values **cannot** be modified
- Only node pointers may be changed
- Works for both even and odd length lists

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)**   |
| Space Complexity  | **O(1)**   |

---
