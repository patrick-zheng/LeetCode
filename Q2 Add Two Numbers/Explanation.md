# LeetCode Problem: Add Two Numbers

- **Problem Link**: [Add Two Numbers – LeetCode](https://leetcode.com/problems/add-two-numbers/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/add-two-numbers/solutions/)

---

## 🧠 Algorithm Explanation

This problem simulates the manual addition of two numbers represented as reversed singly-linked lists, digit by digit.

- Traverse both linked lists simultaneously.
- Add corresponding digits along with any carry from the previous step.
- Store the result as a new node in the output list.
- Carry over any value > 9 to the next digit.
- Continue until both lists and carry are exhausted.

The output is a new linked list that represents the sum, also in reverse order.

---

## ⏱ Time and Space Complexity

- **Time Complexity**: O(max(n, m))
  Where `n` and `m` are the lengths of the two input lists. Each node is visited exactly once.

- **Space Complexity**: O(max(n, m))
  A new node is created for each digit in the result list, which at most equals the longer of the two input lists (plus one if there's a carry).

---
