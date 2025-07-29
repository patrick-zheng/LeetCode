# 🧩 LeetCode Problem: Trapping Rain Water

- **Problem Link**: [Trapping Rain Water – LeetCode](https://leetcode.com/problems/trapping-rain-water/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/trapping-rain-water/solutions/)

---

## 🧠 Algorithm Explanation

We use the **two-pointer approach** to solve the problem in optimal time and space complexity.

The idea is to traverse the `height` array from both ends using two pointers (`left` and `right`). We maintain two variables `left_max` and `right_max` to store the maximum height seen so far from the left and right respectively.

At each step, we:

- Move the pointer at the smaller height.
- If the current height is less than the corresponding max, we trap water at that index.
- Otherwise, we update the max.

This algorithm is used because it reduces space complexity to **O(1)** while maintaining linear **O(n)** time complexity, making it optimal.

---

### 🪜 Steps

1. **Initialize two pointers**: `left = 0`, `right = len(height) - 1`
2. **Initialize two variables**: `left_max = 0`, `right_max = 0`
3. **While `left < right`**:
   - If `height[left] < height[right]`:
     - If `height[left] >= left_max`, update `left_max`
     - Else, add `left_max - height[left]` to result
     - Move `left` pointer to the right
   - Else:
     - If `height[right] >= right_max`, update `right_max`
     - Else, add `right_max - height[right]` to result
     - Move `right` pointer to the left

---

## ✅ Constraints

- `1 <= height.length <= 2 * 10^4`
- `0 <= height[i] <= 10^5`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
