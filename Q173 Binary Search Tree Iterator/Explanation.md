# 🧩 LeetCode Problem: Binary Search Tree Iterator

- **Problem Link**: [Binary Search Tree Iterator – LeetCode](https://leetcode.com/problems/binary-search-tree-iterator/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/binary-search-tree-iterator/solutions/)

---

## 🧠 Algorithm Explanation

We want to iterate through a Binary Search Tree (BST) in **in-order** (left → root → right), which naturally returns values in **sorted order**.

Instead of traversing the entire tree upfront (which would cost O(n) space), we simulate the in-order traversal **lazily** using a **stack**.  
This allows us to return the next smallest element on demand while only storing the path to the next node.

This approach is optimal because:

- It avoids storing all elements.
- Each node is pushed and popped **once**.
- It guarantees **amortized O(1)** time per operation.

---

### 🪜 Steps

1. **Initialization**  
   - Push all left children of the root onto a stack.
   - This positions the iterator at a virtual value smaller than any BST value.

2. **next()**  
   - Pop the top node from the stack (current smallest).
   - If the node has a right child, push all its left descendants onto the stack.
   - Return the node’s value.

3. **hasNext()**  
   - If the stack is not empty, there exists a next element.

---

## ✅ Constraints

- The number of nodes in the BST is in the range `[1, 10⁵]`
- `0 ≤ Node.val ≤ 10⁶`
- Calls to `next()` are always valid
- At most `10⁵` calls will be made to `next()` and `hasNext()`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity                      |
|-------------------|-------------------              |
| Time Complexity   | **O(1)** amortized per `next()` |
| Space Complexity  | **O(h)** (height of the BST)    |

---

### ✅ Why This Is Optimal

- Uses only stack space proportional to tree height
- No modification to the tree
- Clean and intuitive simulation of recursive in-order traversal
