# 🧩 LeetCode Problem: Find First and Last Position of Element in Sorted Array

- **Problem Link**: [Find First and Last Position of Element in Sorted Array – LeetCode](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/)

---

## 🧠 Algorithm Explanation

We are given a sorted array `nums` and a `target` value. We need to find the starting and ending positions of the `target` in the array. If the target is not found, return `[-1, -1]`.

Since the array is sorted, we can apply **binary search** to find both boundaries in `O(log n)` time:

- The **leftmost** (first) occurrence of the target
- The **rightmost** (last) occurrence of the target

Using two slightly different binary search functions ensures precise detection of both bounds efficiently.

---

### 🪜 Steps

1. **Find Left Index**:
   - Use binary search to find the first index where `target` appears.
   - If `nums[mid] < target`, move `left` to `mid + 1`.
   - Otherwise, move `right` to `mid - 1`.

2. **Find Right Index**:
   - Use binary search to find the last index where `target` appears.
   - If `nums[mid] <= target`, move `left` to `mid + 1`.
   - Otherwise, move `right` to `mid - 1`.

3. **Check Validity and Return**:
   - If `left_index <= right_index` and `nums[left_index] == target`, return `[left_index, right_index]`.
   - Otherwise, return `[-1, -1]`.

---

## ✅ Constraints

- `0 <= nums.length <= 10⁵`
- `-10⁹ <= nums[i], target <= 10⁹`
- `nums` is sorted in non-decreasing order.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(log n)   |
| Space Complexity  | O(1)       |

---
