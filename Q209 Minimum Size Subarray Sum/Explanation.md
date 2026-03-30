# 🧩 LeetCode Problem: Minimum Size Subarray Sum

- **Problem Link**: [Minimum Size Subarray Sum – LeetCode](https://leetcode.com/problems/minimum-size-subarray-sum/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/minimum-size-subarray-sum/solutions/)

---

## 🧠 Algorithm Explanation

Since all numbers in `nums` are **positive**, we can use a **sliding window**. This works because:

- Expanding the window to the right always increases the sum.
- Shrinking the window from the left always decreases the sum.
- That lets us efficiently find the smallest subarray whose sum is at least `target`.

This is better than checking every possible subarray, which would be too slow.

---

### 🪜 Steps

1. **Expand the window**  
   Move the right pointer through the array and keep adding values to the current sum.

2. **Shrink when valid**  
   As soon as the current sum becomes greater than or equal to `target`, try shrinking from the left to make the window as small as possible.

3. **Track the minimum length**  
   Each time the window satisfies the condition, update the answer with the smallest length found.

---

## ✅ Constraints

- `1 <= target <= 10^9`
- `1 <= nums.length <= 10^5`
- `1 <= nums[i] <= 10^4`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | `O(n)`     |
| Space Complexity  | `O(1)`     |

---
