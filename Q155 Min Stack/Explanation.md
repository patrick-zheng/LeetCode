# 🧩 LeetCode Problem: Min Stack

- **Problem Link**: [Min Stack – LeetCode](https://leetcode.com/problems/min-stack/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/min-stack/solutions/)

---

## 🧠 Algorithm Explanation

To support **push, pop, top, and getMin in O(1)** time, the stack stores **metadata** with each element.

Instead of storing only values, each stack entry stores:

(value, min_so_far)

Where:

- `value` → actual pushed value
- `min_so_far` → minimum value in the stack up to that point

This guarantees that the minimum element is always available at the top of the stack, enabling constant-time retrieval.

This method avoids:

- Traversal
- Recalculation
- Extra stacks
- Conditional recomputation

---

## 🪜 Steps

1. **Push**
   - If stack is empty → push `(val, val)`
   - Else → push `(val, min(val, current_min))`

2. **Pop**
   - Remove the top element

3. **Top**
   - Return the value part of the top element

4. **GetMin**
   - Return the min_so_far part of the top element

---

## ✅ Constraints

- All operations must run in **O(1)** time
- Stack may contain **negative values**
- `pop`, `top`, and `getMin` are only called on **non-empty stacks**
- Must correctly handle **duplicate minimum values**

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   |    O(1)    |
| Space Complexity  |    O(n)    |

---
