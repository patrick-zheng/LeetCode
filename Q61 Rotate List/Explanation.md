# 🧩 LeetCode Problem: Rotate List

- **Problem Link**: [Rotate List – LeetCode](https://leetcode.com/problems/rotate-list/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/rotate-list/solutions/)

---

## 🧠 Algorithm Explanation

The problem requires rotating a linked list to the right by `k` positions.  
A direct approach would involve moving the last node to the front `k` times,  
but this would result in **O(k*n)** time, which is inefficient for large inputs.  

Instead, we:

1. Compute the length of the list (`n`) and connect the tail to the head to form a cycle.
2. Normalize `k` using `k % n` because rotating `n` times results in the same list.
3. Break the cycle at the correct position to get the rotated list in **O(n)** time.

---

### 🪜 Steps

1. **Handle Edge Cases**  
   If the list is empty, has only one node, or `k == 0`, return the head as-is.

2. **Compute Length and Form a Cycle**  
   Traverse the list to find its length `n` and connect the tail to the head.

3. **Find New Head and Break Cycle**  
   Move `(n - k % n - 1)` steps from the original head to find the new tail.  
   The node after the new tail becomes the new head. Break the cycle.

---

## ✅ Constraints

- The number of nodes in the list is in the range `[0, 500]`.
- `-100 <= Node.val <= 100`
- `0 <= k <= 2 * 10^9`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
