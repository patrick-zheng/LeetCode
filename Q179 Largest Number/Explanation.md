# 🧩 LeetCode Problem: Largest Number

- **Problem Link**: [Largest Number – LeetCode](https://leetcode.com/problems/largest-number/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/largest-number/solutions/)

---

## 🧠 Algorithm Explanation

This problem cannot be solved using normal numeric sorting.  
If we sort numbers in descending numerical order, we may not obtain the largest concatenated result.

For example:

- `9` and `34`
- Numerically: `34 > 9`
- But concatenation:
  - `934` > `349`
  - So `9` should come before `34`

### 🔑 Key Insight

For two numbers converted to strings `a` and `b`:

- If `a + b > b + a`, then `a` should come before `b`
- Otherwise, `b` should come before `a`

This custom comparator ensures the globally optimal concatenated number.

After sorting, we join all numbers into a single string.

### ⚠️ Edge Case

If the largest element after sorting is `"0"`, then all numbers are zeros (e.g., `[0,0]`), so we return `"0"` instead of `"000"`.

---

### 🪜 Steps

1. **Convert Integers to Strings**  
   Convert each number to a string to allow concatenation comparison.

2. **Custom Sort**  
   Sort using a comparator that compares `a + b` and `b + a`.

3. **Handle Leading Zero Case**  
   If the first element is `"0"`, return `"0"`.

4. **Concatenate Result**  
   Join all sorted strings into one final string.

---

## ✅ Constraints

- `1 <= nums.length <= 100`
- `0 <= nums[i] <= 10^9`
- The result may be very large, so it must be returned as a string.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity     |
|-------------------|------------    |
| Time Complexity   | O(n log n * k) |
| Space Complexity  | O(nk)          |

Where:

- `n` = number of elements  
- `k` = average number of digits per number  
- Sorting dominates runtime, and each comparison involves string concatenation.

---
