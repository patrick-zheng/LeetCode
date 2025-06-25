# 🧩 LeetCode Problem: Container With Most Water

- **Problem Link**: [Container With Most Water – LeetCode](https://leetcode.com/problems/container-with-most-water/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/container-with-most-water/solutions/)

---

## 🧠 Algorithm Explanation

The problem is to find the maximum amount of water that can be contained between two vertical lines on a graph, where the heights of the lines are given in an array. The key insight is that the volume of water between any two lines is determined by the **shorter line** and the **distance between them**.

We use a **two-pointer technique**, which reduces the complexity to O(n) by intelligently moving pointers toward each other to find the optimal container.

---

### 🪜 Steps

1. **Initialize** two pointers, one at the start (`left`) and one at the end (`right`) of the height array.
2. **Loop while `left < right`**:
   - Compute the area using the shorter of the two heights times the distance between them.
   - Update the `maxVolume` if the computed area is greater.
   - Move the pointer pointing to the **shorter** height inward.
3. **Return** the `maxVolume` after the loop ends.

---

## ✅ Constraints

- `2 <= height.length <= 10⁵`
- `0 <= height[i] <= 10⁴`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
