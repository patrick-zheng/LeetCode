# 🧩 LeetCode Problem: Kth Largest Element in an Array

- **Problem Link**: [Kth Largest Element in an Array – LeetCode](https://leetcode.com/problems/kth-largest-element-in-an-array/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/)

---

## 🧠 Algorithm Explanation

We are given an integer array `nums` and an integer `k`, and must return the **kth largest** element in the array. The kth largest is defined in sorted **descending** order (the largest element is 1st largest, the second largest is 2nd largest, and so on).

If we sorted `nums` in **ascending** order, the kth largest value would sit at index **`n - k`** (0-based), where **`n = nums.length`**. For example, when `k = 1` we want index `n - 1` (the maximum); when `k = n` we want index `0` (the minimum).

We do **not** need a full sort: we only need the single order statistic at index **`n - k`**. That is a classic **selection** problem. A **partial partition** (the same idea as C++ `std::nth_element`, Rust `select_nth_unstable`, or NumPy `partition`) rearranges the array so that the element at that index is exactly the value it would have if the whole array were sorted, with smaller values on one side and larger on the other. The two sides themselves may remain unsorted.

This approach is used in the C++, Rust, and Python solutions here because it:

- Achieves **expected linear** time in the size of the array for the partition-based selection used by the standard libraries above.
- Uses only **constant extra space** beyond the input (in-place reordering of `nums` / the owned `Vec`).

Other valid strategies include a **size-k min-heap** (**O(n log k)** time, **O(k)** space) or **full sort** (**O(n log n)**). The heap is attractive when `k` is small and you prefer not to mutate the input; partial selection matches `nth_element`-style solutions and stays linear on average for finding one index.

---

### 🪜 Steps

1. **Map “kth largest” to an ascending index**  
   - Let `n = len(nums)`.  
   - The answer is the element that belongs at index **`i = n - k`** in ascending sorted order.

2. **Partially order around index `i`**  
   - **C++**: Call `nth_element` so that the element at position `i` is the correct order statistic; read `nums[i]`.  
   - **Rust**: Take ownership of `nums`, rebind as `let mut nums = nums`, then `select_nth_unstable(n - k)` and use the returned reference to that element.  
   - **Python**: Build an array view with `np.asarray(nums)`, then `np.partition(a, i)` and read `a[i]` (introselect-style partial sort in NumPy).

3. **Return the value**  
   - Cast or dereference as needed so the return type is a plain integer (`int` / `i32`).

---

## ✅ Constraints

- `1 <= k <= nums.length <= 10^5`
- `-10^4 <= nums[i] <= 10^4`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n) average (partial selection / partition; library-dependent constants) |
| Space Complexity  | O(1) auxiliary beyond the input (in-place reorder of `nums`; Python uses a NumPy array derived from `nums`) |

---
