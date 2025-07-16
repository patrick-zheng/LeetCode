# 🧩 LeetCode Problem: Search in Rotated Sorted Array

- **Problem Link**: [Search in Rotated Sorted Array – LeetCode](https://leetcode.com/problems/search-in-rotated-sorted-array/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/)

---

## 🧠 Algorithm Explanation

We use a **modified binary search** because the array is sorted but has been rotated at an unknown pivot. Even after rotation, **at least one half of the array remains sorted**. We can use this property to decide which side of the array to discard and which to search in, achieving the desired **O(log n)** time complexity.

This strategy helps narrow down the search space efficiently by determining which half is sorted and whether the target lies in that half.

---

### 🪜 Steps

1. **Initialize** two pointers: `left = 0`, `right = len(nums) - 1`.
2. **While** `left <= right`:
   - Compute the midpoint: `mid = (left + right) // 2`.
   - If `nums[mid] == target`, return `mid`.
   - Determine which half is sorted:
     - If `nums[left] <= nums[mid]`:
       - Left half is sorted.
       - If `nums[left] <= target < nums[mid]`: search left side.
       - Else: search right side.
     - Else:
       - Right half is sorted.
       - If `nums[mid] < target <= nums[right]`: search right side.
       - Else: search left side.
3. If the loop ends without finding the target, return `-1`.

---

## ✅ Constraints

- `1 <= nums.length <= 5000`
- `-10⁴ <= nums[i] <= 10⁴`
- All values in `nums` are **distinct**
- `nums` is rotated at some pivot
- `-10⁴ <= target <= 10⁴`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(log n)   |
| Space Complexity  | O(1)       |

---
