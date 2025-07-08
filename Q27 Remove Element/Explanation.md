# 🧩 LeetCode Problem: Remove Element

- **Problem Link**: [Remove Element – LeetCode](https://leetcode.com/problems/remove-element/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/remove-element/solutions/)

---

## 🧠 Algorithm Explanation

We need to remove all occurrences of a given value `val` from the input array `nums` *in-place*, and return the number of remaining elements.
The trick is that we don’t actually need to shrink the list itself — we just need to make sure the **first `k` elements of the array** contain the elements not equal to `val`.

We use a **two-pointer technique**:
- One pointer (`read`) scans the entire list.
- Another pointer (`write`) keeps track of the next position to write a valid element.
- When `nums[read] != val`, we write `nums[read]` to `nums[write]` and increment `write`.

Since the problem allows the order of elements to change and the remaining part of the array doesn’t matter, we can also use a **swap-with-last** technique to reduce writes if `val` is rare.

---

## 🪜 Steps

### Method 1: Preserve relative order
1. **Initialize** a `write` pointer at `0`.
2. **Loop through** each element with a `read` pointer.
3. If `nums[read] != val`, overwrite `nums[write] = nums[read]`, then increment `write`.
4. After the loop, return `write` as the count of elements ≠ `val`.

### Method 2: Don’t preserve order (fewer writes if `val` is rare)
1. **Initialize** two pointers: `i = 0` and `n = len(nums)`.
2. While `i < n`:
    - If `nums[i] == val`, swap it with `nums[n-1]` and decrement `n`.
    - Otherwise, increment `i`.
3. Return `n` as the count of elements ≠ `val`.

---

## ✅ Constraints

- `0 <= nums.length <= 100`
- `0 <= nums[i] <= 50`
- `0 <= val <= 100`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
