# 🧩 LeetCode Problem: Copy List with Random Pointer

- **Problem Link**: [Copy List with Random Pointer – LeetCode](https://leetcode.com/problems/copy-list-with-random-pointer/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/copy-list-with-random-pointer/solutions/)

---

## 🧠 Algorithm Explanation

We need to create a **deep copy** of a linked list where each node has:

- `next` pointer (normal list link)
- `random` pointer (can point to any node in the list or `null`)

A clean and safe approach in Rust is to use a **hash map** that maps each original node to its cloned node:

1. **First pass**:  
   - Traverse the original list.
   - For each original node, create a cloned node with the same `val`.
   - Store a mapping:

     ```text
     original_node_ptr (raw pointer) -> cloned Rc<RefCell<Node>>
     ```

2. **Second pass**:  
   - Traverse the original list again.
   - For each original node:
     - Use the map to set the `next` and `random` pointers of the cloned node:
       - `clone.next` is the clone of `original.next`
       - `clone.random` is the clone of `original.random`

3. **Return the cloned head**:  
   - If `head` is `None`, return `None`.
   - Otherwise, look up the cloned node corresponding to `head` and return it.

We use `*const RefCell<Node>` as the key in the `HashMap` so we can uniquely identify each `Rc` node by its raw pointer. This avoids needing `Eq/Hash` on `Rc`.

---

### 🪜 Steps

1. **Step 1: Clone all nodes (values only)**
   - Iterate with `curr = head.clone()`.
   - For each node:
     - Create `Rc<RefCell<Node>>` with same `val`.
     - Insert it into the map: `map[Rc::as_ptr(&curr)] = clone`.

2. **Step 2: Connect `next` and `random` for the cloned nodes**
   - Iterate again from `head`.
   - For each original node:
     - Get its corresponding cloned node from the map.
     - If `original.next` exists, set `clone.next` to the clone of that node.
     - If `original.random` exists, set `clone.random` to the clone of that node.

3. **Step 3: Return cloned head**
   - If `head` is `None` → return `None`.
   - Else, return `map[ptr_of(head)]`.

---

## ✅ Constraints

- `0 <= n <= 1000`
- `-10^4 <= Node.val <= 10^4`
- `Node.random` is `null` or points to some node in the list.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |

We visit each node a constant number of times and store one entry per node in the map.

---
