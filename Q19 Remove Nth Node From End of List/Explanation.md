# 🧩 LeetCode Problem: Remove Nth Node From End of List

- **Problem Link**: [Remove Nth Node From End of List – LeetCode](https://leetcode.com/problems/remove-nth-node-from-end-of-list/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/)

---

## 🧠 Algorithm Explanation

We use a two-pointer technique to solve this problem efficiently. The main idea is to maintain two pointers: a `fast` pointer and a `slow` pointer. The `fast` pointer moves `n+1` steps ahead of the `slow` pointer. Then, both pointers are moved together until the `fast` pointer reaches the end of the list. The `slow` pointer will be pointing just before the node that needs to be removed, so we adjust the links to remove the node.

This approach ensures that we only traverse the list once, achieving optimal time complexity.

---

### 🪜 Steps

1. **Step 1**: Create a dummy node that points to the head of the list to handle edge cases like removing the head node.
2. **Step 2**: Move the `fast` pointer `n+1` steps ahead.
3. **Step 3**: Move both the `fast` and `slow` pointers until the `fast` pointer reaches the end. The `slow` pointer will now be just before the node that needs to be removed.
4. **Step 4**: Adjust the `next` pointer of the `slow` pointer to remove the node.

---

## ✅ Constraints

- The number of nodes in the list is in the range [1, 10^5].
- `1 <= n <= length of the list`.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(N)       |
| Space Complexity  | O(1)       |

---
