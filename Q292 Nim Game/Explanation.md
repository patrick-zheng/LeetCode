# LeetCode Problem: Nim Game

- **Problem Link**: [Nim Game – LeetCode](https://leetcode.com/problems/nim-game/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/nim-game/solutions/)

---

## Algorithm

There are `n` stones. On each turn you may remove **1**, **2**, or **3**
stones. The player who takes the last stone wins. Return **true** if you
can force a win with optimal play.

### Losing positions

If the pile size is a multiple of **4**, the current player loses against
optimal play: whatever you take (1–3), the opponent can take enough to leave
another multiple of 4.

If `n % 4 != 0`, take `n % 4` stones first and leave a multiple of 4 for the
opponent.

So the answer is simply `n % 4 != 0`.

---

## Constraints

- `1 <= n <= 2^31 - 1`

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(1)** — one modulo check                                                     |
| Space Complexity   | **O(1)** — constant extra space                                                 |

---
<!-- markdownlint-enable MD013 -->
