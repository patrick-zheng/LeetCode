# 🧩 LeetCode Problem: Search in Rotated Sorted Array II

- **Problem Link**: [Search in Rotated Sorted Array II – LeetCode](https://leetcode.com/problems/search-in-rotated-sorted-array-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/search-in-rotated-sorted-array-ii/solutions/)

---

## 🧠 Algorithm Explanation

We need to determine whether a target exists in a rotated sorted array that may contain duplicates.  
A **modified binary search** is used. Normally, in rotated arrays, one half is guaranteed to be sorted, which helps decide which side to discard. However, duplicates complicate this because equal values can make it ambiguous which side is sorted.

To handle this:

- When `nums[l] == nums[m] == nums[r]`, we can’t determine which half is sorted, so we shrink the search space by incrementing `l` and decrementing `r`.
- Otherwise, we check whether the left or right half is sorted and move the pointers accordingly.
- If `nums[m] == target`, we immediately return `True`.

This preserves average **O(log n)** behavior while ensuring correctness in the presence of duplicates.

---

### 🪜 Steps

1. **Initialize pointers**  
   Set `l = 0`, `r = len(nums) - 1`.

2. **Binary search loop**  
   While `l <= r`, compute `m = (l + r) // 2`.

3. **Check for match**  
   If `nums[m] == target`, return `True`.

4. **Handle duplicates**  
   If `nums[l] == nums[m] == nums[r]`, shrink both ends (`l += 1`, `r -= 1`).

5. **Check sorted half**  
   - If left half is sorted (`nums[l] <= nums[m]`), check if `target` lies within it.  
   - Else, the right half is sorted, check if `target` lies within it.

6. **Adjust search space**  
   Move `l` or `r` based on the above check.

7. **Not found**  
   If the loop ends, return `False`.

---

## ✅ Constraints

- `1 <= nums.length <= 5000`
- `-10^4 <= nums[i] <= 10^4`
- `nums` is sorted in non-decreasing order, then rotated
- `-10^4 <= target <= 10^4`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(log n) on average, O(n) in worst case (all duplicates) |
| Space Complexity  | O(1) |

---
