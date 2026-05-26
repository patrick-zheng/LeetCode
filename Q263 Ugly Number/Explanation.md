# LeetCode Problem: Ugly Number

- **Problem Link**: [Ugly Number – LeetCode](https://leetcode.com/problems/ugly-number/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/ugly-number/solutions/)

---

## Algorithm

An **ugly number** is a **positive** integer whose only prime factors are
**2**, **3**, and **5**. Equivalently, after removing every factor of 2, 3,
and 5, nothing is left except **1**.

1. If `n <= 0`, return **false** (not positive).
2. For each prime `p` in `{2, 3, 5}`, while `n` is divisible by `p`, divide
   `n` by `p`.
3. Return **true** iff the remaining value is **1**. If any other prime
   divided `n`, the remainder is greater than 1 (e.g. `14` → `7`).

No factorization table or recursion is needed; each division strictly shrinks
`n`.

---

## Constraints

- `-2^31 <= n <= 2^31 - 1`
- Ugly numbers are defined as **positive**; non-positive inputs are **false**

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(log n)** — each division at least halves `n` for factor 2; few steps total  |
| Space Complexity   | **O(1)** — only the running value of `n`                                        |

---
<!-- markdownlint-enable MD013 -->
