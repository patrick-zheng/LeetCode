# 🧩 LeetCode Problem: Linked List Cycle

- **Problem Link**: [Linked List Cycle – LeetCode](https://leetcode.com/problems/linked-list-cycle/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/linked-list-cycle/solutions/)

---

## 🧠 Algorithm Explanation

This problem is solved using **Floyd’s Cycle Detection Algorithm**, also known as the **Tortoise and Hare** approach.

Two pointers traverse the linked list at different speeds:

- A **slow pointer** moves one node at a time
- A **fast pointer** moves two nodes at a time

If the list contains a cycle, the fast pointer will eventually meet the slow pointer.  
If there is no cycle, the fast pointer will reach the end of the list.

This approach is optimal because it detects cycles without modifying the list and uses constant extra memory.

---

## 🪜 Steps

1. **Initialize pointers**
   - Set both `slow` and `fast` to the head of the linked list.

2. **Traverse the list**
   - Move `slow` one step forward.
   - Move `fast` two steps forward.
   - Continue while `fast` and `fast.next` are not `None`.

3. **Detect a cycle**
   - If `slow == fast`, a cycle exists → return `True`.
   - If the loop exits, no cycle exists → return `False`.

---

## ✅ Constraints

- Number of nodes: `0 ≤ n ≤ 10⁴`
- Node values: `-10⁵ ≤ Node.val ≤ 10⁵`
- The linked list must not be modified

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | `O(n)`     |
| Space Complexity  | `O(1)`     |
