# 🧩 LeetCode Problem: Maximum Gap

- **Problem Link**: [Maximum Gap – LeetCode](https://leetcode.com/problems/maximum-gap/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/maximum-gap/solutions/)

---

## 🧠 Algorithm Explanation

This problem requires finding the **maximum difference between two successive elements** in the sorted version of an array, under the strict constraints that the algorithm must run in **linear time O(n)** and use **linear extra space O(n)**.

Because traditional sorting algorithms require **O(n log n)** time, they violate the problem constraints.  
Therefore, we use a **bucket-based algorithm** derived from the **Pigeonhole Principle**.

### Key Insight

If `n` numbers lie between `min` and `max`, the **minimum possible maximum gap** is:

\[
\left\lceil \frac{max - min}{n - 1} \right\rceil
\]

This means:

- The largest gap **cannot occur inside a bucket**
- It must occur **between buckets**

So instead of sorting, we:

- Distribute values into buckets
- Track min/max in each bucket
- Compute gaps between successive non-empty buckets

---

## 🪜 Steps

1. **Handle Edge Case**  
   If the array has fewer than 2 elements, return `0`.

2. **Find Range**  
   Compute the global `min` and `max` of the array.

3. **Create Buckets**  
   - Compute bucket size using:
     \[
     \text{bucketSize} = \left\lceil \frac{max - min}{n - 1} \right\rceil
     \]
   - Create `n - 1` buckets, each storing:
     - minimum value
     - maximum value

4. **Distribute Values**  
   Place each number into its appropriate bucket based on its value.

5. **Compute Maximum Gap**  
   Traverse buckets in order and compute:
   - `current.bucketMin - previous.bucketMax`
   - Track the maximum difference

---

## ✅ Constraints

- `1 ≤ nums.length ≤ 10⁵`
- `0 ≤ nums[i] ≤ 10⁹`
- Must run in **O(n)** time
- Must use **O(n)** extra space
- No sorting allowed

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)**   |
| Space Complexity  | **O(n)**   |

---

## 🧠 Why This Algorithm Is Optimal

- Avoids comparison-based sorting
- Uses distribution instead of ordering
- Guarantees correctness via mathematical lower bound on gap size
- Meets strict linear constraints
- Scales efficiently for large datasets

---
