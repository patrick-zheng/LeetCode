# 🧩 LeetCode Problem: Merge Sorted Array

- **Problem Link**: [Merge Sorted Array – LeetCode](https://leetcode.com/problems/merge-sorted-array/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/merge-sorted-array/solutions/)

---

## 🧠 Algorithm Explanation

We are given two sorted arrays `nums1` and `nums2`.  
The task is to merge them into `nums1` while keeping it sorted in non-decreasing order.  

Key challenge: `nums1` has extra trailing space (of size `n`) to hold all elements from `nums2`.  
We must merge **in-place** without returning a new array.  

**Optimal Strategy**:  
Merge from the **end of both arrays** to avoid overwriting elements in `nums1`.  

- Compare the last valid elements of `nums1` and `nums2`.  
- Place the larger one at the last available slot in `nums1`.  
- Move the pointers backwards.  
- If any elements remain in `nums2`, copy them into `nums1`.  

This ensures an **O(m+n)** merge with **O(1)** extra space.

---

### 🪜 Steps

1. Initialize three pointers:  
   - `i = m - 1` (last valid element in `nums1`)  
   - `j = n - 1` (last element in `nums2`)  
   - `k = m + n - 1` (last slot in `nums1`)  

2. While both arrays still have elements:  
   - Compare `nums1[i]` and `nums2[j]`.  
   - Place the larger value at `nums1[k]`.  
   - Decrement the respective pointer (`i` or `j`) and `k`.  

3. If `nums2` has leftover elements, copy them into `nums1`.  

---

## ✅ Constraints

- `nums1.length == m + n`  
- `nums2.length == n`  
- `0 <= m, n <= 200`  
- `1 <= m + n <= 200`  
- `-10^9 <= nums1[i], nums2[j] <= 10^9`  
- Both `nums1` and `nums2` are sorted in non-decreasing order.  

---

## ⏱ Time and Space Complexity

| Metric            | Complexity   |
|-------------------|--------------|
| Time Complexity   | **O(m + n)** |
| Space Complexity  | **O(1)**     |

---
