# LeetCode Problem: Two Sum

- **Problem Link**: [Two Sum – LeetCode](https://leetcode.com/problems/two-sum/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/two-sum/solutions/)

---

## 🧠 Algorithm Approaches

### 1. Brute Force

- **Description**: Check every pair of elements in the array to see if their sum matches the target.
- **Time Complexity**: O(n²)
- **Space Complexity**: O(1)
- **Use Case**: Simple to implement, but inefficient for large input sizes.

---

### 2. Two Pointers on Sorted Array

- **Description**: Sort the array while tracking original indices, then use two pointers from both ends to find the sum.
- **Time Complexity**: O(n log n) due to sorting
- **Space Complexity**: O(n) for storing original indices
- **Use Case**: Useful if a sorted array is acceptable or reused elsewhere.

---

### 3. Hash Map Lookup

- **Description**: Iterate once through the list, storing seen numbers in a hash map and checking for the complement.
- **Time Complexity**: O(n)
- **Space Complexity**: O(n)
- **Use Case**: Fastest and most commonly used approach for this problem.

---
