# 🧩 LeetCode Problem: Largest Rectangle in Histogram

- **Problem Link**: [Largest Rectangle in Histogram – LeetCode](https://leetcode.com/problems/largest-rectangle-in-histogram/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/largest-rectangle-in-histogram/solutions/)

---

## 🧠 Algorithm Explanation

We need to find the area of the largest rectangle that can fit entirely under the histogram.  
The most optimal approach uses a **monotonic increasing stack**:

- The stack keeps indices of bars in ascending order of height.  
- When we encounter a bar shorter than the one on top of the stack, we pop from the stack and compute the largest rectangle possible with that bar as the smallest height.  
- To ensure all bars are processed, we add sentinel `0` heights at both ends.

This ensures each bar is pushed and popped at most once, giving an **O(n)** runtime.

---

### 🪜 Steps

1. **Preprocessing**: Add sentinel `0` bars at the beginning and end of the histogram to handle edge cases cleanly.
2. **Stack Maintenance**: Iterate through each bar. Push indices while the current bar is taller than the top of the stack.
3. **Area Calculation**:  
   - If the current bar is shorter, pop from the stack.  
   - Compute area with the popped bar as the limiting height:  
     `area = height[popped] * (current_index - stack[-1] - 1)`.  
   - Update `max_area` accordingly.
4. **Result**: Return the maximum area found.

---

## ✅ Constraints

- `1 <= len(heights) <= 10^5`
- `0 <= heights[i] <= 10^4`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)** — each bar is pushed and popped at most once |
| Space Complexity  | **O(n)** — for the stack and sentinel-augmented array |

---
