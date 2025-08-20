# 🧩 LeetCode Problem: Insert Interval

- **Problem Link**: [Insert Interval – LeetCode](https://leetcode.com/problems/insert-interval/)
- **Solution Link**: [Official/Community Solutions](https://leetcode.com/problems/insert-interval/solutions/)

---

## 🧠 Algorithm Explanation

Do a single left-to-right pass using three phases, leveraging that `intervals` is **sorted by start** and **non-overlapping**:

1) Append all intervals that end **before** `newInterval.start` (no overlap).  
2) **Merge** the contiguous block of intervals that **overlap** `newInterval` by expanding `[start, end]` to the min/max bounds seen.  
3) Append all intervals that start **after** the merged interval.

Because overlaps (if any) form one contiguous block, a single linear scan suffices.

---

### 🪜 Steps

1. **Add non-overlapping (before)**  
   While `intervals[i][1] < newStart`, push `intervals[i]` and advance `i`.

2. **Merge overlaps**  
   While `i < n` and `intervals[i][0] <= newEnd`, set  
   `newStart = min(newStart, intervals[i][0])` and  
   `newEnd   = max(newEnd, intervals[i][1])`, then advance `i`.  
   After the loop, push `[newStart, newEnd]`.

3. **Add non-overlapping (after)**  
   Push all remaining intervals from `i` to `n - 1`.

---

## ✅ Constraints

- `0 ≤ intervals.length ≤ 10^4`
- `intervals[i].length == 2`
- `0 ≤ start_i ≤ end_i ≤ 10^5`
- `newInterval.length == 2`, `0 ≤ start ≤ end ≤ 10^5`
- `intervals` is sorted by start and contains **no overlaps**
- You may return a **new array** (no in-place modification required)

---

## ⏱ Time and Space Complexity

| Metric            | Complexity                        |
|-------------------|-----------------------------------|
| Time Complexity   | O(n)                              |
| Space Complexity  | O(1) extra (O(n) including output) |

---
