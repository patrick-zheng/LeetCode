# 🧩 LeetCode Problem: Happy Number

- **Problem Link**: [Happy Number – LeetCode](https://leetcode.com/problems/happy-number/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/happy-number/solutions/)

---

## 🧠 Algorithm Explanation

A number is **happy** if repeatedly replacing it with the **sum of the squares of its digits** eventually leads to `1`.

The key observation is that this process can only do one of two things:

1. Reach `1`, which means the number is happy.
2. Fall into a cycle, which means it will repeat forever and never reach `1`.

To detect that cycle efficiently, we use **Floyd’s Cycle Detection Algorithm (Tortoise and Hare)** instead of using a hash set.

- `slow` moves one step at a time.
- `fast` moves two steps at a time.

If the sequence enters a cycle, `slow` and `fast` will eventually meet.
If `fast` reaches `1`, then the number is happy.

This algorithm is used because it gives the optimal result with **constant extra space**.

---

### 🪜 Steps

1. **Compute the next number** by summing the squares of the digits of the current number.

2. **Use two pointers**:
   - `slow` moves one transformation at a time.
   - `fast` moves two transformations at a time.

3. **Stop when**:
   - `fast == 1`, meaning the number is happy.
   - `slow == fast`, meaning a cycle was detected and the number is not happy.

---

## ✅ Constraints

- `1 <= n <= 2^31 - 1`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(log n)   |
| Space Complexity  | O(1)       |

---
