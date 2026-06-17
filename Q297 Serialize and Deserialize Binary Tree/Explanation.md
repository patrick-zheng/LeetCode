# LeetCode Problem: Serialize and Deserialize Binary Tree

- **Problem Link**: [Serialize and Deserialize Binary Tree – LeetCode](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/solutions/)

---

## Algorithm

Design `Codec` with `serialize(root)` and `deserialize(data)` that round-trip a
binary tree.

### Preorder DFS with null markers

Walk the tree in **preorder** (node, left, right). Write each node value as
text; write `N` for `null`. Join tokens with commas.

Example: `[1, 2, 3, null, null, 4, 5]` becomes
`1,2,N,N,3,4,N,N,5,N,N`.

**`deserialize(data)`**

1. Split `data` on commas.
2. Rebuild with the same preorder walk: read one token at a time.
3. If token is `N`, return `null`; otherwise create the node, then attach
   `left` and `right` recursively.

Preorder uniquely encodes the tree shape and values. Both passes are linear in
the number of nodes.

---

## Constraints

- The number of nodes is in the range `[0, 10^4]`
- `-1000 <= Node.val <= 1000`

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** — each node visited once in serialize and deserialize                  |
| Space Complexity   | **O(n)** — output string and recursion stack (or token list)                    |

---
<!-- markdownlint-enable MD013 -->
