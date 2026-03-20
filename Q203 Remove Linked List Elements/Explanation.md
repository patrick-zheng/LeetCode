# 🧩 LeetCode Problem: 203. Remove Linked List Elements

- **Problem Link**: [203. Remove Linked List Elements – LeetCode](https://leetcode.com/problems/remove-linked-list-elements/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/remove-linked-list-elements/solutions/)

---

## 🧠 Algorithm Explanation

The optimal solution uses a **dummy node** placed before the head of the linked list. This is useful because the nodes that need to be removed may include the original head itself, and using a dummy node avoids special-case handling for that situation.

We then traverse the list with a pointer called `cur`. At each step, we check `cur.next`:

- If `cur.next.val == val`, we remove that node by skipping it: `cur.next = cur.next.next`
- Otherwise, we move `cur` forward

This works in one pass and modifies the list in-place.

---

### 🪜 Steps

1. **Create a dummy node** that points to `head` so removal of the first real node is handled cleanly.

2. **Traverse the list** using a pointer starting from the dummy node.

3. **Remove matching nodes** by reconnecting pointers whenever `cur.next.val == val`; otherwise continue moving forward.

---

## ✅ Constraints

- The number of nodes in the list is in the range `[0, 10^4]`
- `1 <= Node.val <= 50`
- `0 <= val <= 50`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
