# 🧩 LeetCode Problem: Two Sum II – Input Array Is Sorted

- **Problem Link**: [Two Sum II – Input Array Is Sorted – LeetCode](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/solutions/)

---

## 🧠 Algorithm Explanation

Because the array is already sorted in **non-decreasing order**, we can use a **two-pointer technique** instead of a hash map.  
This approach maintains **constant extra space** while scanning the array efficiently in one pass.

By placing one pointer at the beginning and one at the end, we can compare the sum of the two values and move pointers intelligently based on whether the sum is too small or too large.

---

### 🪜 Steps

1. **Initialize two pointers**  
   - `left` at the start of the array  
   - `right` at the end of the array

2. **Check the sum**
   - If `numbers[left] + numbers[right] == target`, return the indices (1-indexed).
   - If the sum is **less than target**, move `left` to the right.
   - If the sum is **greater than target**, move `right` to the left.

3. **Repeat until solution is found**  
   - The problem guarantees **exactly one valid solution**, so the loop will always terminate.

---

## ✅ Constraints

- The array is already **sorted**
- Exactly **one solution exists**
- Indices must be **1-indexed**
- The same element **cannot be used twice**
- Must use **O(1) extra space**

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)**   |
| Space Complexity  | **O(1)**   |

---
