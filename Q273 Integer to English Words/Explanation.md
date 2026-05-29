# LeetCode Problem: Integer to English Words

- **Problem Link**: [Integer to English Words – LeetCode](https://leetcode.com/problems/integer-to-english-words/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/integer-to-english-words/solutions/)

---

## Algorithm

Convert `num` to English words for `0 <= num <= 2^31 - 1`. Brute-force
digit-by-digit rules are error-prone; the standard approach is **chunking by
thousands**.

### Special case

- `num == 0` → `"Zero"`.

### Helper: 0–999

Map any chunk `n` in `1..=999` with fixed word tables:

- **1–19**: `One` … `Nineteen`
- **20–99**: tens word + optional ones (`Twenty Three`)
- **100–999**: hundreds word + optional remainder (`Three Hundred Four`)

Return `""` for chunk `0` so empty middle groups (e.g. `1_000_010`) are skipped.

### Thousands scale

Process four 3-digit groups from least significant:

`[ones, Thousand, Million, Billion]`

For each group index `i`, if `num % 1000 != 0`, append
`chunk_to_words(num % 1000)` plus the scale label (none for the ones group).
Then `num /= 1000`. Join groups **high to low** with single spaces.

This is **O(1)** in input size (at most four chunks, each bounded work).

---

## Constraints

- `0 <= num <= 2^31 - 1`
- No leading/trailing spaces; correct capitalization and spelling per LeetCode

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(1)** — at most four 3-digit chunks, each converted in constant time         |
| Space Complexity   | **O(1)** — output length is bounded; auxiliary storage is constant              |

---
<!-- markdownlint-enable MD013 -->
