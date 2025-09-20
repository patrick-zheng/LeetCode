# 🧩 LeetCode Problem: Remove Duplicates from Sorted Array II

- **Problem Link**: [Remove Duplicates from Sorted Array II – LeetCode](https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solutions/)

---

## 🧠 Algorithm Explanation

We need to remove duplicates from a sorted array **in-place** such that each unique element appears at most **twice**, while maintaining the relative order. The challenge is to perform this with **O(1) extra space**.

The optimal approach uses the **two-pointer technique**:

- A **write pointer (`w`)** tracks the position to write the next valid number.
- As we iterate with a **read pointer**, we check if the current number should be written.
- A number is valid if either:
  1. Fewer than two elements have been written so far, or  
  2. The current number is **different** from the number at index `w-2`.

This ensures that no number appears more than twice.

---

### 🪜 Steps

1. **Initialize write index (`w = 0`)**  
   Start from the beginning of the array.

2. **Iterate through each number**  
   For each element `x` in the array:
   - If `w < 2` (first two numbers) → always keep it.
   - Otherwise, keep it only if `x != nums[w-2]`.

3. **Write valid elements in place**  
   When valid, assign `nums[w] = x` and increment `w`.

---

## ✅ Constraints

- The array is sorted in non-decreasing order.
- Modify the array in-place with O(1) extra memory.
- Return the new length `k`, where the first `k` elements are valid.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
