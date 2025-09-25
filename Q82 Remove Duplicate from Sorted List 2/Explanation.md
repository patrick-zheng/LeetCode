# 🧩 LeetCode Problem: Remove Duplicates from Sorted List II

- **Problem Link**: [Remove Duplicates from Sorted List II – LeetCode](https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/solutions/)

---

## 🧠 Algorithm Explanation

We are given a **sorted linked list** and need to remove **all nodes that have duplicate values**, leaving only distinct numbers in sorted order.

Because the list is sorted, duplicates always appear as **consecutive runs**.  
We can detect such runs and skip them entirely, instead of just removing extra nodes.

We use a **dummy head** and two pointers (`prev` and `cur`):

- `cur` iterates through the list.
- If `cur` points to a duplicate run, skip the entire run and link `prev.next` to the node after the run.
- Otherwise, move `prev` forward when a unique node is found.

This ensures only unique elements remain.

---

### 🪜 Steps

1. **Create a dummy node** pointing to the head. This handles edge cases where the head itself is removed.
2. **Iterate with `cur`**:
   - If `cur` and `cur.next` have the same value, store this value and skip all nodes with that value.
   - Link `prev.next` to the first node after the run.
3. **If no duplicate is found**, move both `prev` and `cur` forward.
4. Continue until `cur` reaches the end.
5. Return `dummy.next` as the new head.

---

## ✅ Constraints

- The number of nodes in the list is in the range `[0, 300]`.
- `-100 <= Node.val <= 100`
- The list is sorted in **non-decreasing** order.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n) – each node is visited once |
| Space Complexity  | O(1) – no extra data structures used |

---
