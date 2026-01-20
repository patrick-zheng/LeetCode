# 🧩 LeetCode Problem: Maximum Product Subarray

- **Problem Link**: [Maximum Product Subarray – LeetCode](https://leetcode.com/problems/maximum-product-subarray/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/maximum-product-subarray/solutions/)

---

## 🧠 Algorithm Explanation

This problem asks for the **maximum product of a contiguous subarray**.  
Unlike sum-based subarray problems, products are tricky because:

- A **negative number** can turn a large negative product into a large positive one.
- A **zero** resets any running product.
- The maximum product at an index depends on **both** the maximum and minimum products from the previous index.

To handle this, we use **Dynamic Programming** while keeping track of:

- `maxEndingHere`: maximum product ending at the current index
- `minEndingHere`: minimum product ending at the current index

Tracking both is essential because multiplying by a negative value swaps their roles.

---

### 🪜 Steps

1. **Initialize**
   - Set `maxEndingHere`, `minEndingHere`, and `result` to the first element.

2. **Iterate through the array**
   - If the current number is negative, swap `maxEndingHere` and `minEndingHere`.
   - Update:
     - `maxEndingHere = max(current, current × maxEndingHere)`
     - `minEndingHere = min(current, current × minEndingHere)`

3. **Update result**
   - Keep the maximum value seen so far using `result = max(result, maxEndingHere)`.

---

## ✅ Constraints

- `1 ≤ nums.length ≤ 2 × 10⁴`
- `-10 ≤ nums[i] ≤ 10`
- The final answer fits in a **32-bit integer**

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)**   |
| Space Complexity  | **O(1)**   |

---
