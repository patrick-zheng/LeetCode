# 🧩 LeetCode Problem: Intersection of Two Linked Lists

- **Problem Link**: <https://leetcode.com/problems/intersection-of-two-linked-lists/>
- **Solution Link**: <https://leetcode.com/problems/intersection-of-two-linked-lists/solutions/>

---

## 🧠 Algorithm Explanation

This problem asks us to find the **node reference** where two singly linked lists intersect (not the value, but the actual memory node).  
The key constraint is that the lists must remain unchanged and there are no cycles.

The optimal solution uses a **two-pointer synchronization technique**:

- Traverse both lists with two pointers.
- When a pointer reaches the end of one list, redirect it to the head of the other list.
- This equalizes path lengths automatically.

### Why this works

If list A has length `a`, list B has length `b`, and the shared tail has length `c`, then:

- Pointer A walks: `a + b` nodes  
- Pointer B walks: `b + a` nodes  

So both pointers traverse the **same total distance**, guaranteeing:

- They meet at the intersection node, or  
- They both reach `null` at the same time (no intersection)

No length calculation, no memory, no modification to the list structure.

---

### 🪜 Steps

1. **Initialize two pointers**  
   - `pA = headA`  
   - `pB = headB`

2. **Traverse the lists**
   - Move both pointers one step at a time
   - If a pointer reaches `null`, redirect it to the other list’s head

3. **Detect intersection**
   - When `pA == pB`, that node is the intersection
   - If both become `null`, there is no intersection

---

## ✅ Constraints

- Lists must retain original structure
- No cycles in the linked lists
- Intersection is defined by **node reference**, not value
- Judge builds shared structure using `skipA` and `skipB`
- Must return the actual intersecting node
- If no intersection exists → return `null`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity   |
|-------------------|--------------|
| Time Complexity   | **O(n + m)** |
| Space Complexity  | **O(1)**     |

---
