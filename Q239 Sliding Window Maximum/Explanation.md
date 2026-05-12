# LeetCode Problem: Sliding Window Maximum

- **Problem Link**: [Sliding Window Maximum – LeetCode][problem]
- **Solution Link**: [Official Solutions][solutions]

[problem]: https://leetcode.com/problems/sliding-window-maximum/
[solutions]: https://leetcode.com/problems/sliding-window-maximum/solutions/

---

## Algorithm

For each window of length `k`, we need the **maximum** value. A naive scan of
each window costs **O(n·k)** and is too slow for `n` up to \(10^5\).

### Monotonic deque (indices)

Keep a **deque of indices** into `nums` such that corresponding values are in
**strictly decreasing** order from front to back. The **front** always holds
the index of the **current maximum** inside the window.

For each position `i` from left to right:

1. **Drop expired indices:** while the front index is **outside** the window
   ending at `i` (index \(\le i - k\)), pop from the **front**.
2. **Maintain decreasing order:** while the back index’s value is \(\le\)
   `nums[i]`, pop from the **back** (those elements can never be the max while
   `i` is in the window).
3. **Push** `i` at the **back**.
4. Once `i >= k - 1`, append `nums[front]` to the answer.

Each index is **pushed once** and **popped at most once** from each end, so the
total work is **linear** in `n`.

---

## Constraints

- `1 <= nums.length <= 10^5`
- `-10^4 <= nums[i] <= 10^4`
- `1 <= k <= nums.length`

---

<!-- markdownlint-disable MD013 -->

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** — monotonic deque; each index enqueued/dequeued at most once           |
| Space Complexity   | **O(k)** — deque holds at most k indices (candidates for window maximum)        |

<!-- markdownlint-enable MD013 -->

---
