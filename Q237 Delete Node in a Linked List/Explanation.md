# LeetCode Problem: Delete Node in a Linked List

- **Problem Link**: [Delete Node in a Linked List - LeetCode](https://leetcode.com/problems/delete-node-in-a-linked-list/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/delete-node-in-a-linked-list/solutions/)

---

## Algorithm

The input gives us a direct pointer/reference to the node that should be deleted,
but not the head of the list. Since we cannot reach the previous node, we cannot
remove `node` in the usual way (`prev.next = node.next`).

The key observation is that `node` is guaranteed **not** to be the tail.
Therefore, `node.next` always exists.

### Key idea

Replace the current node's value with the next node's value, then bypass the next
node:

1. `node.val = node.next.val`
2. `node.next = node.next.next`

This makes the current node look exactly like its next node used to look, and the
original next node is removed from the chain.

---

## Constraints

- The number of nodes in the linked list is in the range `[2, 1000]`.
- `-1000 <= Node.val <= 1000`
- The value of each node in the list is unique.
- The node to be deleted is in the list and is not a tail node.

---

## Complexity

| Metric           | Complexity                              |
|------------------|-----------------------------------------|
| Time Complexity  | **O(1)** with constant-time updates.    |
| Space Complexity | **O(1)** with no extra data structures. |

---
