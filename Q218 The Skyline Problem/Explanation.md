# 🧩 LeetCode Problem: The Skyline Problem

- **Problem Link**: [The Skyline Problem – LeetCode](https://leetcode.com/problems/the-skyline-problem/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/the-skyline-problem/solutions/)

---

## 🧠 Algorithm Explanation

The skyline only changes at building **edges**: when a wall starts, the maximum height among active buildings can go up; when a wall ends, it can go down. So we use a **sweep line** over increasing `x` and maintain the set of **currently active** building heights.

Each building `[left, right, height]` becomes two events:

- **Start** at `x = left`: the building becomes active.
- **End** at `x = right`: the building stops (LeetCode uses half-open intervals: the building covers `[left, right)`, so the right edge is excluded from the silhouette).

We encode events as pairs `(x, h)`:

- **Left edge**: store `(left, -height)` — negative height marks a start.
- **Right edge**: store `(right, height)` — positive height marks an end.

After sorting lexicographically:

1. At the same `x`, **all starts come before all ends** (negative second key before positive), which matches the rule that a new building starting at `x` meets the skyline before an old one finishes at `x`.
2. Among starts at the same `x`, **taller buildings are processed first** (more negative `-height` sorts earlier), so the running maximum updates in the right order.

We **group all events with the same `x`**, apply every add/remove for that `x`, then read the current maximum height. We append `[x, max_height]` to the answer only if it differs from the last key point’s height (avoids duplicate consecutive heights at the same level).

### Maintaining the maximum among active heights

- **Python**: a min-heap of **negative** heights acts as a max-heap; **lazy deletion** (`removed[height]`) handles ends, then we pop stale tops until the top is still active.
- **C++ / Rust**: a **sorted map** from height to count (`std::map` / `BTreeMap`) stores how many active buildings have each height; the current max is the largest key with a positive count (map’s last entry). Ground level is kept by initializing height `0` with count `1`.

---

### 🪜 Steps

1. **Build events** `(left, -height)` and `(right, height)` for every building and sort.
2. **Sweep** in order of `x`; for each distinct `x`, apply every event at that coordinate (add height on start, decrement/remove on end).
3. **Current skyline height** = max over active heights (heap top after pruning, or map’s max key).
4. **Emit** `[x, current_height]` if the height changed from the previous key point.

---

## ✅ Constraints

- `1 <= buildings.length <= 10^4`
- Each building is `[left_i, right_i, height_i]` with `1 <= left_i < right_i <= 2^31 - 1`, `1 <= height_i <= 2^31 - 1`, and `right_i - left_i >= 1`

---

## ⏱ Time and Space Complexity

| Metric           | Complexity                                                                                         |
|------------------|----------------------------------------------------------------------------------------------------|
| Time Complexity  | O(n log n), where n is the number of buildings (2n events, each heap/map operation is logarithmic) |
| Space Complexity | O(n) for events and the active-height structure                                                    |

---
