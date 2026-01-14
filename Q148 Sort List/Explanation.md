# 🧩 LeetCode Problem: Sort List

- **Problem Link**: [Sort List – LeetCode](https://leetcode.com/problems/sort-list/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/sort-list/solutions/)

---

## 🧠 Algorithm Explanation

The problem asks us to sort a **singly linked list** in ascending order.  
Because linked lists do **not support random access**, comparison-based sorting algorithms like quicksort or heapsort are inefficient or cumbersome.

The optimal solution is **Merge Sort**, which is well-suited for linked lists because:

- It does not require random access
- It runs in `O(n log n)` time
- It can be implemented with `O(1)` extra space (excluding recursion stack)

Merge Sort works by recursively splitting the list into halves, sorting each half, and merging them back together in sorted order.

---

### 🪜 Steps

1. **Split the linked list into two halves**  
   - Use the **slow and fast pointer technique**  
   - When `fast` reaches the end, `slow` will be at the midpoint  
   - Cut the list into two separate lists

2. **Recursively sort both halves**  
   - Apply merge sort to the left half  
   - Apply merge sort to the right half  

3. **Merge the two sorted halves**  
   - Compare nodes from both lists  
   - Build a new sorted list by always selecting the smaller node  

---

## ✅ Constraints

- The number of nodes in the list is in the range `[0, 5 * 10⁴]`
- `-10⁵ ≤ Node.val ≤ 10⁵`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity                           |
|-------------------|--------------------------------------|
| Time Complexity   | **O(n log n)**                       |
| Space Complexity  | **O(1)** (excluding recursion stack) |

---
