# 🧩 LeetCode Problem: Sort Colors

- **Problem Link**: [Sort Colors – LeetCode](https://leetcode.com/problems/sort-colors/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/sort-colors/solutions/)

---

## 🧠 Algorithm Explanation

This problem is a classic example of the **Dutch National Flag problem**.  
We need to sort an array consisting only of `0`, `1`, and `2` **in-place** without using any extra sorting functions.  

The idea is to use **three pointers**:

- `low` → tracks the position where the next `0` should go.
- `mid` → scans through the array.
- `high` → tracks the position where the next `2` should go.

By swapping elements appropriately, we ensure:

- All `0`s are before `low`.
- All `1`s are between `low` and `high`.
- All `2`s are after `high`.

This avoids the need for counting or extra storage, keeping it **O(n)**.

---

### 🪜 Steps

1. **Initialize pointers**:  
   - `low = 0`, `mid = 0`, `high = len(nums) - 1`.

2. **Traverse the array with `mid`**:  
   - If `nums[mid] == 0`: swap `nums[mid]` with `nums[low]`, increment both `low` and `mid`.
   - If `nums[mid] == 1`: just move `mid` forward.
   - If `nums[mid] == 2`: swap `nums[mid]` with `nums[high]`, decrement `high` (but don’t increment `mid` yet, since we need to re-check the swapped element).

3. **Stop when `mid > high`**:  
   - At this point, the array is sorted in-place.

---

## ✅ Constraints

- `1 <= nums.length <= 300`
- `nums[i]` is either `0`, `1`, or `2`
- Must sort in-place without using built-in sort.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)** — each element is processed at most once. |
| Space Complexity  | **O(1)** — only a few pointers are used. |

---
