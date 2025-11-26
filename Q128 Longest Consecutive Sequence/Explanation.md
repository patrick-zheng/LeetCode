# 🧩 LeetCode Problem: Longest Consecutive Sequence

- **Problem Link**: [Longest Consecutive Sequence – LeetCode](https://leetcode.com/problems/longest-consecutive-sequence/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/longest-consecutive-sequence/solutions/)

---

## 🧠 Algorithm Explanation

We are given an unsorted array of integers `nums` and need to return the length of the **longest consecutive elements sequence**, with an overall time complexity of **O(n)**.

A straightforward approach would be to sort the array and then scan it to find the longest chain of consecutive numbers. However, sorting takes **O(n log n)**, which does **not** meet the problem’s **O(n)** requirement.

To get **O(n)**, we use a **hash set** to support **O(1)** average-time lookups:

1. Insert all numbers into a set.
2. For each number `x` in the set, we only consider it a potential **start** of a sequence if `x - 1` is **not** in the set.
   - This ensures we don’t repeatedly recount sequences from the middle.
3. Starting from each such `x`, we repeatedly check whether `x + 1`, `x + 2`, ... exist in the set and count the length of this consecutive run.
4. Track and return the maximum length seen.

This works in linear time because:

- Each number is checked at most once as a potential sequence start.
- Each number is visited at most once during the “expansion” of some sequence.

We choose this algorithm because it:

- Satisfies the **O(n)** time requirement.
- Is conceptually simple and only needs extra **O(n)** space for the set.
- Avoids sorting or more complex data structures.

---

### 🪜 Steps

1. **Handle edge cases**  
   - If `nums` is empty, return `0` immediately.

2. **Build a hash set**  
   - Insert all elements of `nums` into a set `num_set` for constant-time membership checks.

3. **Find sequence starts**  
   - Iterate over each `x` in `num_set`.
   - If `x - 1` is **not** in `num_set`, then `x` is the **start** of a consecutive sequence.

4. **Expand each sequence**  
   - For each start `x`, initialize:
     - `current = x`
     - `length = 1`
   - While `current + 1` is in `num_set`:
     - Increment `current`
     - Increment `length`

5. **Track the maximum**  
   - Maintain a variable `longest` to store the maximum `length` found so far.
   - After iterating through all numbers, return `longest`.

---

## ✅ Constraints

- `0 <= nums.length <= 10^5`
- `-10^9 <= nums[i] <= 10^9`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |

---
