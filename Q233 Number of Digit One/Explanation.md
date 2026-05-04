# LeetCode Problem: Number of Digit One

- **Problem Link**: [Number of Digit One – LeetCode](https://leetcode.com/problems/number-of-digit-one/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/number-of-digit-one/solutions/)

---

## Algorithm

Count how many times the digit **1** appears in the decimal representation of
every integer from **1** through **\(n\)**. Enumerating each number and counting
digits is **O(n log n)** and is too slow for **\(n\)** up to **\(10^9\)**.

### Per-position (factor) formula

For each power **factor** in **1, 10, 100, …** while **factor ≤ n**, look at the
digit at that place when **\(n\)** is written in base 10. Split **\(n\)** into
three parts (all integer division / remainder):

- **higher** = `n // (factor * 10)` — all digits to the **left** of the current
  place
- **current** = `(n // factor) % 10` — the digit at this place
- **lower** = `n % factor` — all digits to the **right** of the current place

The number of **1**s contributed at this place is:

- If **current = 0**: **higher × factor** (all “full” blocks of **factor**
  numbers where this digit was 1)
- If **current = 1**: **higher × factor + lower + 1** (full blocks before the
  partial tail, then **1** through the tail)
- If **current > 1**: **(higher + 1) × factor** (an extra full block of
  **factor** numbers with a **1** in this place)

For **n = 0** the answer is **0**; use **64-bit** intermediate values (C++ /
Rust) so products like **higher × factor** do not overflow (the statement’s
hint calls this out).

---

## Constraints

- `0 <= n <= 10^9`

---

## Complexity

<!-- markdownlint-disable MD013 -->

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(log n)** — constant work per base-10 digit                                  |
| Space Complexity   | **O(1)** — only a few scalar variables                                          |

<!-- markdownlint-enable MD013 -->

---
