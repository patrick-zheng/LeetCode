# 🧩 LeetCode Problem: Problem Name

- **Problem Link**: [Problem Name – LeetCode](https://leetcode.com/problems/problem-name/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/problem-name/solutions/)

---

## 🧠 Algorithm Explanation

This problem requires finding a **peak element** (an element strictly greater than its neighbors) in **O(log n)** time, which immediately suggests using **binary search** instead of linear scanning.

Key Insight:

If you compare `nums[mid]` with `nums[mid + 1]`:

- If `nums[mid] < nums[mid + 1]`, you're on an **ascending slope** → a peak **must exist on the right side**.
- Otherwise, you're on a **descending slope or at a peak** → a peak **must exist on the left side (including mid)**.

Because the problem guarantees:

`nums[-1] = nums[n] = -∞`

A peak **always exists**, so binary search is safe and valid.

This makes the problem a classic **binary search on monotonic slope** problem.

---

### 🪜 Steps

1. **Initialize two pointers**
   - `l = 0`, `r = n - 1`

2. **Binary search loop**
   - Compute `mid = (l + r) // 2`
   - Compare `nums[mid]` with `nums[mid + 1]`

3. **Move search space**
   - If `nums[mid] < nums[mid + 1]` → move right: `l = mid + 1`
   - Else → move left: `r = mid`

4. **Stop condition**
   - When `l == r`, that index is a **peak element**

---

## ✅ Constraints

- `1 <= nums.length <= 1000`
- `-2^31 <= nums[i] <= 2^31 - 1`
- Adjacent elements are **not equal**
- Must run in **O(log n)** time

---

## ⏱ Time and Space Complexity

| Metric            |  Complexity |
|-------------------|-------------|
| Time Complexity   | **O(logn)** |
| Space Complexity  | **O(1)**    |

---
