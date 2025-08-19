# 🧩 LeetCode Problem: Merge Intervals

- **Problem Link**: [Merge Intervals – LeetCode](https://leetcode.com/problems/merge-intervals/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/merge-intervals/solutions/)

---

## 🧠 Algorithm Explanation

We need to merge all overlapping intervals in a given list.  
The key insight is that **sorting by start times** ensures that any possible overlaps are adjacent, making it efficient to merge them in a single pass.  

This algorithm is chosen because:

- Sorting ensures correct ordering of intervals.
- A single linear scan is sufficient to detect and merge overlaps.
- It achieves the optimal time complexity of **O(n log n)**.

---

### 🪜 Steps

1. **Sort intervals** by their start value.  
2. **Initialize** the first interval as the current `[start, end]`.  
3. **Iterate** through the remaining intervals:
   - If the next interval overlaps (its start ≤ current end), extend the current end.
   - Otherwise, push the current interval to the result and start a new one.  
4. **Add** the last interval to the result.  
5. **Return** the merged list.  

---

## ✅ Constraints

- `1 <= intervals.length <= 10^4`  
- `intervals[i].length == 2`  
- `0 <= starti <= endi <= 10^4`  

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n log n) |
| Space Complexity  | O(1) extra (ignoring output) |

---
