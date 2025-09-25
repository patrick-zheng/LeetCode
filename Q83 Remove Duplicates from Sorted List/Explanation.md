# 🧩 LeetCode Problem: Remove Duplicates from Sorted List

- **Problem Link**: [Remove Duplicates from Sorted List – LeetCode](https://leetcode.com/problems/remove-duplicates-from-sorted-list/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/remove-duplicates-from-sorted-list/solutions/)

---

## 🧠 Algorithm Explanation

The problem requires us to remove duplicates from a **sorted linked list** so that each element appears only once.  
Since the list is already sorted, duplicates will always appear next to each other.  

We can simply iterate through the list and compare the current node with the next node. If they are the same, we skip the duplicate node by adjusting the pointer. Otherwise, we move to the next node.  

This works efficiently because:

- No extra data structures are needed.
- The traversal is done only once.

---

### 🪜 Steps

1. **Initialize a pointer** `current` to the head of the linked list.  
2. **Traverse the list** while `current` and `current.next` are not `None`.  
3. **Check for duplicates**:  
   - If `current.val == current.next.val`, remove the duplicate by setting `current.next = current.next.next`.  
   - Otherwise, move `current` to the next node.  
4. **Return the head** of the modified list.  

---

## ✅ Constraints

- The number of nodes in the list is in the range `[0, 300]`.  
- `-100 <= Node.val <= 100`  
- The list is guaranteed to be **sorted in non-decreasing order**.  

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)** – Each node is visited once |
| Space Complexity  | **O(1)** – No extra memory used |

---
