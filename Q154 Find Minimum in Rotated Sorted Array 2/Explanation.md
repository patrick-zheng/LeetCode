# 🧩 LeetCode Problem: Find Minimum in Rotated Sorted Array II

- **Problem Link**: [Find Minimum in Rotated Sorted Array II – LeetCode](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/solutions/)

---

## 🧠 Algorithm Explanation

This problem is solved using a **modified binary search**.  
Because the array is originally sorted and then rotated, the minimum element is located at the **rotation pivot**.

However, unlike the simpler version of this problem, the array **may contain duplicates**, which introduces ambiguity in comparisons and prevents strict binary partitioning in all cases.

To handle this efficiently:

We compare the middle element with the rightmost element:

- If `nums[mid] < nums[r]`, the minimum lies in the **left half (including mid)**.
- If `nums[mid] > nums[r]`, the minimum lies in the **right half (excluding mid)**.
- If `nums[mid] == nums[r]`, duplicates prevent us from knowing which side contains the pivot, so we **safely shrink the search space** by decrementing `r`.

This keeps the algorithm optimal in practice while handling duplicate values correctly.

---

### 🪜 Steps

1. **Initialize pointers**  
   Set `left = 0` and `right = n - 1`.

2. **Binary decision loop**  
   While `left < right`:
   - Compute `mid = (left + right) // 2`
   - Compare `nums[mid]` with `nums[right]`:
     - If `nums[mid] < nums[right]` → `right = mid`
     - If `nums[mid] > nums[right]` → `left = mid + 1`
     - Else (`nums[mid] == nums[right]`) → `right -= 1`

3. **Return result**  
   When `left == right`, the pointer is at the minimum element → return `nums[left]`.

---

## ✅ Constraints

- `1 ≤ nums.length ≤ 5000`
- `-10^4 ≤ nums[i] ≤ 10^4`
- Array is originally sorted in ascending order
- Array is rotated between `1` and `n` times
- **Duplicates are allowed**
- Must minimize total operations

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   |   `O(n)`   |
| Space Complexity  |   `O(1)`   |

---
