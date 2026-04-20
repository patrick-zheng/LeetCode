# LeetCode Problem: Rectangle Area

- **Problem Link**: [Rectangle Area – LeetCode](https://leetcode.com/problems/rectangle-area/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/rectangle-area/solutions/)

---

## Algorithm

Each rectangle is axis-aligned. Rectangle **A** has bottom-left corner
(`ax1`, `ay1`) and top-right corner (`ax2`, `ay2`), with `ax1 < ax2` and
`ay1 < ay2`. Rectangle **B** is described the same way using `bx1`, `by1`,
`bx2`, `by2`.

The total covered area is the **union** of the two rectangles (overlap
counted once):

```text
area(A ∪ B) = area(A) + area(B) − area(A ∩ B)
```

Each rectangle’s area is width × height:

```text
area(A) = (ax2 − ax1) × (ay2 − ay1)
area(B) = (bx2 − bx1) × (by2 − by1)
```

The intersection (when it exists) is also an axis-aligned rectangle. Take
the overlap of the two x-intervals and the two y-intervals:

```text
ix1 = max(ax1, bx1)    iy1 = max(ay1, by1)
ix2 = min(ax2, bx2)    iy2 = min(ay2, by2)
```

If `ix1 < ix2` **and** `iy1 < iy2`, overlap is `(ix2 - ix1) * (iy2 - iy1)`.
Otherwise overlap is `0`.

Return `area(A) + area(B) − overlap`. In C++ and Rust, use **`i64` /
`long long`** for products so intermediate values do not overflow `i32`.

---

## Constraints

- `-10^4 <= ax1, ay1, ax2, ay2, bx1, by1, bx2, by2 <= 10^4`
- `ax1 < ax2`, `ay1 < ay2`, `bx1 < bx2`, `by1 < by2`

---

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(1)** — fixed min/max and a few integer products                             |
| Space Complexity   | **O(1)** — only scalar intermediates                                            |

---
