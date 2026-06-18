# LeetCode Problem: Bulls and Cows

- **Problem Link**: [Bulls and Cows – LeetCode](https://leetcode.com/problems/bulls-and-cows/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/bulls-and-cows/solutions/)

---

## Algorithm

Given equal-length digit strings `secret` and `guess`:

- **Bulls (A)** — same digit in the same position.
- **Cows (B)** — digit appears in both strings but in the wrong position
  (not already counted as a bull).

Return `"{bulls}A{cows}B"`.

### One pass + frequency buckets

1. Scan index by index. If `secret[i] == guess[i]`, increment bulls.
2. Otherwise count `secret[i]` and `guess[i]` in two length-10 frequency
   arrays (digits `0`–`9`).
3. For each digit `d`, add `min(secret_freq[d], guess_freq[d])` to cows.

Bulls are excluded from the frequency arrays, so cows are not double-counted.

Example: `secret = "1807"`, `guess = "7810"` → `"1A3B"`.

---

## Constraints

- `1 <= secret.length, guess.length <= 1000`
- `secret.length == guess.length`
- `secret` and `guess` consist of digits only

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** — one pass over the strings; digit buckets are size 10                 |
| Space Complexity   | **O(1)** — two fixed-size frequency arrays                                      |

---
<!-- markdownlint-enable MD013 -->
