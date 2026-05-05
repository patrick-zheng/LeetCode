# LeetCode Problem: Palindrome Linked List

- **Problem Link**: [Palindrome Linked List - LeetCode](https://leetcode.com/problems/palindrome-linked-list/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/palindrome-linked-list/solutions/)

---

## Algorithm

The optimal approach runs in linear time and constant extra space:

1. Use two pointers (`slow`, `fast`) to find the middle of the linked list.
2. Reverse the second half in-place.
3. Compare the first half and the reversed second half node by node.
4. If every compared pair matches, the list is a palindrome.

Why this works:

- `slow` reaches the midpoint when `fast` reaches the end.
- Reversing only the second half lets us compare mirrored positions directly.
- We only use a fixed number of pointers, so extra space is `O(1)`.

---

## Constraints

- The number of nodes in the list is in the range `[1, 10^5]`
- `0 <= Node.val <= 9`

---

## Complexity

| Metric             | Complexity                      |
|--------------------|---------------------------------|
| Time Complexity    | **O(n)**                        |
| Space Complexity   | **O(1)**                        |

---
