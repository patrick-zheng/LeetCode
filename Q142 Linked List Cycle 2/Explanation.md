# 🧩 LeetCode Problem: Linked List Cycle II

- **Problem Link**: [Linked List Cycle II – LeetCode](https://leetcode.com/problems/linked-list-cycle-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/linked-list-cycle-ii/solutions/)

---

## 🧠 Algorithm Explanation

This solution uses **Floyd’s Tortoise and Hare algorithm** (two-pointer technique).

The idea is:

- If a cycle exists, a **slow pointer** (1 step at a time) and a **fast pointer** (2 steps at a time) will eventually meet inside the cycle.
- Once they meet, resetting one pointer to the head and moving both pointers **one step at a time** will cause them to meet again **exactly at the entry point of the cycle**.

This approach is optimal because it:

- Detects the cycle without extra memory
- Finds the cycle’s starting node in linear time

---

### 🪜 Steps

1. **Step 1: Initialize two pointers**
   - `slow` and `fast` both start at the head of the linked list.

2. **Step 2: Detect if a cycle exists**
   - Move `slow` by one step and `fast` by two steps.
   - If they ever meet, a cycle exists.
   - If `fast` or `fast.next` becomes `None`, there is no cycle.

3. **Step 3: Find the cycle entry point**
   - Reset `slow` to the head.
   - Move both `slow` and `fast` one step at a time.
   - The node where they meet is the **start of the cycle**.

---

## ✅ Constraints

- The number of nodes in the list is in the range `[0, 10⁴]`
- Node values do not affect the algorithm
- The list may or may not contain a cycle
- **No extra space** may be used

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)**   |
| Space Complexity  | **O(1)**   |

---

## 💡 Key Insight

If:

- `L` = distance from head to cycle entry  
- `C` = cycle length  

When `slow` and `fast` meet:

- Resetting `slow` to head works because both pointers are exactly `L` steps away from the cycle entry.

This mathematical property is what makes Floyd’s algorithm elegant and efficient.
