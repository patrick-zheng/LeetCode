# LeetCode Problem: Product of Array Except Self

- **Problem Link**: [Product of Array Except Self – LeetCode][problem]
- **Solution Link**: [Official Solutions][solutions]

[problem]: https://leetcode.com/problems/product-of-array-except-self/
[solutions]: https://leetcode.com/problems/product-of-array-except-self/solutions/

---

## Algorithm

For each index `i`, we need the product of all elements **except**
`nums[i]`. Division is disallowed, and we want **O(n)** time with **O(1)**
extra space besides the output array.

### Prefix and suffix products

Write the answer at index `i` as **answer[i]** = (product of every
`nums[j]` with `j < i`) × (product of every `nums[j]` with `j > i`).

The **left** factor is a **prefix product** up to (but not including) `i`;
the **right** factor is a **suffix product** after `i`.

1. **First pass (left to right):** fill the output with prefix products.
   After the pass, `result[i]` equals the product of all `nums[j]` with
   `j < i`.
2. **Second pass (right to left):** keep a running **suffix** product.
   Multiply `result[i]` by that suffix, then extend the suffix by
   `nums[i]`.

No auxiliary array is needed beyond the return value; only **O(1)**
scalars (`prefix` / `suffix` or equivalent) are used.

---

## Constraints

- `2 <= nums.length <= 10^5`
- `-30 <= nums[i] <= 30`
- The product of any prefix or suffix of `nums` is guaranteed to fit in a
  **32-bit** integer.
- **O(n)** time; write **O(1)** extra space (the output array does not count
  as extra space).

---

<!-- markdownlint-disable MD013 -->

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** — two passes: prefix products, then multiply by suffix                 |
| Space Complexity   | **O(1)** extra — output array excluded; only a few scalars                      |

<!-- markdownlint-enable MD013 -->

---
