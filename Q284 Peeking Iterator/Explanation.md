# LeetCode Problem: Peeking Iterator

- **Problem Link**: [Peeking Iterator – LeetCode](https://leetcode.com/problems/peeking-iterator/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/peeking-iterator/solutions/)

---

## Algorithm

Extend an existing iterator with **`peek()`**, which returns the next value
without advancing the read position.

### Look-ahead cache

Keep one buffered element from the underlying iterator:

1. **Constructor**: if the base iterator has a next value, read it into
   `peeked` / `cached`.
2. **`peek()`**: return the cached value.
3. **`next()`**: return the cache, then refill it from the base iterator if
   more elements exist.
4. **`hasNext()`**: true while the cache holds a value.

Each call is **O(1)** because we only read at most one new element from the
base iterator.

Example on `[1, 2, 3]`: after `next()` returns `1`, `peek()` returns `2`
without consuming it; the next `next()` still returns `2`.

---

## Constraints

- `1 <= nums.length <= 1000`
- `1 <= nums[i] <= 1000`
- All calls to `next` and `peek` are valid
- At most `1000` calls to `next`, `hasNext`, and `peek`

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(1)** per `peek`, `next`, and `hasNext`                                      |
| Space Complexity   | **O(1)** — one cached element besides the wrapped iterator                      |

---
<!-- markdownlint-enable MD013 -->
