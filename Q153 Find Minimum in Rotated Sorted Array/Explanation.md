# 🧩 LeetCode Problem: Find Minimum in Rotated Sorted Array

- **Problem Link**: [Find Minimum in Rotated Sorted Array – LeetCode](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/solutions/)

---

## 🧠 Algorithm Explanation

The array was originally sorted in ascending order, then rotated.  
This means **one half of the array is always sorted**, while the other may contain the rotation pivot (the minimum element).

To achieve **O(log n)** time complexity, we use **binary search** instead of scanning the array linearly.

Key observation:

- If `nums[mid] > nums[right]`, the minimum lies **to the right of `mid`**.
- Otherwise, the minimum lies **at `mid` or to the left of it**.

By repeatedly narrowing the search space, we locate the minimum efficiently.

---

### 🪜 Steps

1. **Initialize pointers**
   - Set `left = 0`, `right = n - 1`.

2. **Binary search loop**
   - Compute `mid = (left + right) // 2`.
   - If `nums[mid] > nums[right]`, move `left = mid + 1`.
   - Else, move `right = mid`.

3. **Return result**
   - When `left == right`, it points to the minimum element.

---

## ✅ Constraints

- `1 ≤ nums.length ≤ 5000`
- `-5000 ≤ nums[i] ≤ 5000`
- All elements in `nums` are **unique**
- `nums` is a rotated version of a strictly increasing sorted array

---

## ⏱ Time and Space Complexity

| Metric           | Complexity   |
|------------------|--------------|
| Time Complexity  | **O(log n)** |
| Space Complexity |   **O(1)**   |

---
