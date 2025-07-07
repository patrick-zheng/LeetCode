# 🧩 LeetCode Problem: Remove Duplicates from Sorted Array

- **Problem Link**: [Remove Duplicates from Sorted Array – LeetCode](https://leetcode.com/problems/remove-duplicates-from-sorted-array/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/remove-duplicates-from-sorted-array/solutions/)

---

## 🧠 Algorithm Explanation

We are given a **sorted** array and asked to remove duplicates **in-place**, returning the length of the deduplicated array.
Because the array is sorted, all duplicates appear consecutively — so we can use the **two-pointer technique** to overwrite duplicates as we find them while iterating once through the array.

This algorithm is chosen because:
- It runs in **O(n) time**, which is optimal because we must inspect every element at least once.
- It uses **O(1) extra space**, since it modifies the array in-place without creating additional data structures.

---

## 🪜 Steps

1. **Initialize a write pointer (`k`) at index 0**
   This points to where the next unique number should be written.

2. **Iterate through the array with a read pointer**
   For each number, if it’s different from the last written unique number (`nums[k-1]`), we write it at `nums[k]` and increment `k`.

3. **Return `k` as the length of the deduplicated array**
   After one pass, the first `k` elements of `nums` are the unique elements in order.

---

## ✅ Constraints

- `1 <= nums.length <= 3 * 10⁴`
- `-100 <= nums[i] <= 100`
- `nums` is sorted in **non-decreasing order**

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
