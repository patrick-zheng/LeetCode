# 🧩 LeetCode Problem: Next Permutation

- **Problem Link**: [Next Permutation – LeetCode](https://leetcode.com/problems/next-permutation/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/next-permutation/solutions/)

---

## 🧠 Algorithm Explanation

The algorithm generates the **next lexicographical permutation** of the given array in-place. If the current array is the last permutation (i.e., sorted in descending order), it transforms the array into the **first permutation** (i.e., sorted in ascending order).

We use a **greedy strategy**, working from the back of the array to find the first pair that violates descending order, then make a minimal swap to get the next permutation, and finally reverse the suffix to make it the smallest possible.

---

### 🪜 Steps

1. **Step 1: Find the first decreasing element from the end**  
   Traverse from right to left and find the first index `i` such that `nums[i] < nums[i + 1]`.  
   If no such index exists, the array is in descending order (last permutation).

2. **Step 2: Find the element just larger than `nums[i]`**  
   Traverse from the end and find the first index `j` such that `nums[j] > nums[i]`.

3. **Step 3: Swap `nums[i]` and `nums[j]`**

4. **Step 4: Reverse the subarray from `i + 1` to the end**  
   This ensures the suffix is the smallest possible.

---

## ✅ Constraints

- `1 <= nums.length <= 100`
- `0 <= nums[i] <= 100`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
