# 🧩 LeetCode Problem: First Missing Positive

- **Problem Link**: [First Missing Positive – LeetCode](https://leetcode.com/problems/first-missing-positive/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/first-missing-positive/solutions/)

---

## 🧠 Algorithm Explanation

We use the **cyclic sort** strategy to place each integer `x` (if `1 ≤ x ≤ n`) at the index `x - 1`. This helps us rearrange the array in linear time without using extra space.  
After sorting, the first index where `nums[i] != i + 1` indicates the smallest missing positive integer.

This approach avoids the overhead of negative marking and absolute value calculations, making it one of the fastest implementations in practice.

---

### 🪜 Steps

1. **Initialize pointer `i = 0`**.
2. **Swap values into their correct positions**: While `nums[i]` is in the range `[1, n]` and not already in the correct place, swap `nums[i]` with `nums[nums[i] - 1]`.
3. **After rearrangement**, scan the array:
   - The first index `i` where `nums[i] != i + 1` is the answer.
   - If all are correctly placed, return `n + 1`.

---

## ✅ Constraints

- `1 <= nums.length <= 10⁵`
- `-2³¹ <= nums[i] <= 2³¹ - 1`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
