# 🧩 LeetCode Problem: Swap Nodes in Pairs

- **Problem Link**: [Swap Nodes in Pairs – LeetCode](https://leetcode.com/problems/swap-nodes-in-pairs/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/swap-nodes-in-pairs/solutions/)

---

## 🧠 Algorithm Explanation

We traverse the linked list and swap every two adjacent nodes by changing their pointers.
We use a dummy node to simplify handling of the head node and ensure all swaps happen in-place without allocating new nodes.

We chose an **iterative approach with a dummy node** because:
- It’s O(n) time and O(1) space.
- No recursion overhead.
- Clean and easy to implement.

---

### 🪜 Steps

1. **Initialize**:
   - Create a dummy node that points to the head of the list.
   - Use a `curr` pointer starting at the dummy.

2. **Iterate through the list**:
   - While `curr.next` and `curr.next.next` are not `None`, we have a pair to swap.
   - Identify the first and second nodes in the pair.

3. **Swap the nodes**:
   - Change pointers so that the second node points to the first, and the first points to the next pair.
   - Link `curr` to the second node (which becomes the new first in this pair).

4. **Move to the next pair**:
   - Advance `curr` by two nodes (since the first node is now after the second).

---

## ✅ Constraints

- The number of nodes in the list is in the range [0, 100].
- `0 <= Node.val <= 100`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
