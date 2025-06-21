# LeetCode Problem: Median of Two Sorted Arrays

- **Problem Link**: [Median of Two Sorted Arrays – LeetCode](https://leetcode.com/problems/median-of-two-sorted-arrays/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/median-of-two-sorted-arrays/solutions/)

---

## 🧠 Algorithm Explanation

This problem is solved using a **binary search** approach to partition the two sorted arrays such that:

- The left half contains the same number of elements as the right half (or one more if the total number of elements is odd).
- Every element in the left half is less than or equal to every element in the right half.

### Steps

- Always perform binary search on the **shorter array** (`nums1`) for efficiency.
- At each step, partition `nums1` at index `partitionA`, and compute the corresponding partition `partitionB` in `nums2`.
- Check if the max of the left partitions (`maxLeftA`, `maxLeftB`) is less than or equal to the min of the right partitions (`minRightA`, `minRightB`).
- If valid, compute the median:
  - If total length is even, it's the average of the two middle values.
  - If odd, it's the maximum of the left partition.
- If invalid:
  - Move the binary search left or right depending on which side violates the partition condition.

---

## ⏱ Time and Space Complexity

- **Time Complexity**: O(log(min(m, n)))
  Binary search is applied only on the smaller array.

- **Space Complexity**: O(1)
  Only a few pointers and scalar variables are used.

---
