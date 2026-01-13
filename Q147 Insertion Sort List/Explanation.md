# 🧩 LeetCode Problem: Insertion Sort List

- Problem Link: <https://leetcode.com/problems/insertion-sort-list/>
- Solution Link: <https://leetcode.com/problems/insertion-sort-list/solutions/>

---

## 🧠 Algorithm Explanation

This problem requires sorting a singly linked list using insertion sort.

Insertion sort maintains a sorted portion of the list and iteratively inserts each new node into its correct position within that sorted portion. For linked lists, insertion is efficient once the correct position is found because no element shifting is required.

Why insertion sort?

- The problem explicitly asks for insertion sort
- Linked lists support efficient insertions
- Sorting is done in-place with constant extra space
- Performs well on nearly sorted lists

---

## 🪜 Steps

1. Initialize helpers  
   - Create a dummy node before the head to simplify edge cases  
   - last_sorted tracks the end of the sorted portion  
   - curr is the node currently being inserted  

2. Traverse the list  
   - If curr.val ≥ last_sorted.val, the list is still sorted → move forward  
   - Otherwise, search for the correct insertion position starting from dummy  

3. Insert the node  
   - Remove curr from its current position  
   - Insert it between prev and prev.next  
   - Continue with the next unsorted node  

---

## ✅ Constraints

- Number of nodes: 0 ≤ n ≤ 5000  
- Node values: -5000 ≤ Node.val ≤ 5000  
- Singly linked list  

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n²)      |
| Space Complexity  | O(1)       |

---
