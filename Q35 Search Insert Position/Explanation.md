# 🧩 LeetCode Problem: Search Insert Position

- **Problem Link**: [Search Insert Position – LeetCode](https://leetcode.com/problems/search-insert-position/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/search-insert-position/solutions/)

---

## 🧠 Algorithm Explanation

This problem asks for the index where a target value should be inserted in a sorted array so that the array remains in order. If the target already exists in the array, return its index.

To solve this efficiently, we use **Binary Search**, which reduces the time complexity from O(n) to O(log n). Binary search repeatedly divides the search space in half based on comparisons.

---

### 🪜 Steps

1. **Initialize** two pointers: `left = 0`, `right = len(nums) - 1`.
2. **Loop** while `left <= right`:
   - Calculate `mid = (left + right) // 2`.
   - If `nums[mid] == target`, return `mid`.
   - If `nums[mid] < target`, shift search space right by setting `left = mid + 1`.
   - If `nums[mid] > target`, shift search space left by setting `right = mid - 1`.
3. **Return** `left` when the loop ends — this is the correct insert position.

---

## ✅ Python Code

```python
class Solution:
    def searchInsert(self, nums: list[int], target: int) -> int:
        left, right = 0, len(nums) - 1

        while left <= right:
            mid = (left + right) // 2

            if nums[mid] == target:
                return mid
            elif nums[mid] < target:
                left = mid + 1
            else:
                right = mid - 1

        return left
