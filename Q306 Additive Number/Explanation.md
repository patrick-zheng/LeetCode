# LeetCode Problem: Additive Number

- **Problem Link**: [Additive Number – LeetCode](https://leetcode.com/problems/additive-number/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/additive-number/solutions/)

---

## Algorithm

A string is an **additive number** if its digits can be split into a sequence
of at least **three** integers where (after the first two) each term equals the
sum of the two before it. No number may have a leading zero unless it is `"0"`.

### Try the first two numbers, then verify greedily

1. Choose end indices `i` and `j` for the first and second numbers
   (`num[0..i]`, `num[i..j]`).
2. Reject splits with illegal leading zeros.
3. While characters remain, the next number must equal `a + b` as a digit
   string; advance the pointer by that length.
4. Success if the whole string is consumed.

Example: `"112358"` → `1`, `1`, `2`, `3`, `5`, `8`.

Example: `"199100199"` → `1`, `99`, `100`, `199`.

Use string addition in C++/Rust so large partial sums never overflow fixed-width
integers.

---

## Constraints

- `1 <= num.length <= 35`
- `num` consists only of digits

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n^3)** — O(n^2) first-two splits; each check is O(n) in length              |
| Space Complexity   | **O(n)** — temporary sum strings during verification                            |

---
<!-- markdownlint-enable MD013 -->
