# LeetCode Problem: Power of Two

- **Problem Link**: [Power of Two – LeetCode](https://leetcode.com/problems/power-of-two/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/power-of-two/solutions/)

---

## Algorithm

We must decide whether **`n`** equals \(2^k\) for some integer \(k \ge 0\).

### Bit trick — **O(1)** time, **O(1)** space

Positive powers of two have **exactly one** `1` in binary (e.g.
\(16 = 10000_2\)). If **`n`** is such a number, then **`n - 1`** clears that
leading `1` and sets all lower bits to `1` (e.g. \(15 = 01111_2\)). Their
bitwise **AND** is zero:

\[
n \mathbin{\&} (n - 1) = 0
\]

Any **`n > 0`** that is **not** a power of two has **at least two** `1` bits,
so the **AND** is nonzero. The case **`n = 0`** must be rejected explicitly
(it would also satisfy the **AND** identity).

**Test:** return **`n > 0`** and `` `(n & (n - 1)) == 0` ``.

**Rust:** cast **`n`** to **`u32`** before subtracting so the mask is
well-defined for all **`i32`** values with **`n > 0`**.

---

## Constraints

- `-2^31 <= n <= 2^31 - 1`

---

<!-- markdownlint-disable MD013 -->

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(1)** — constant-time bitwise check on the input integer                     |
| Space Complexity   | **O(1)** — only fixed scalar operations; no extra structures                    |

<!-- markdownlint-enable MD013 -->

---
