# 🧩 LeetCode Problem: Maximum Subarray

- **Problem Link**: [Maximum Subarray – LeetCode](https://leetcode.com/problems/maximum-subarray/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/maximum-subarray/solutions/)

---

## 🧠 Algorithm Explanation

We use **Kadane’s Algorithm**, which finds the largest sum of any contiguous subarray in linear time.  
The algorithm works by maintaining a running sum of the subarray we are currently considering and resetting it whenever starting a new subarray would yield a larger sum.  
This is optimal because every element must be examined at least once, making \(O(n)\) the best possible time complexity.

---

### 🪜 Steps

1. **Initialize**:
   - Set `max_sum` to the first element (this will store the maximum sum found so far).
   - Set `current_sum` to the first element (this will store the current subarray sum).

2. **Iterate through the array** (starting from the second element):
   - Update `current_sum` to be the larger of:
     - The current element (`num`)
     - `current_sum + num` (extending the current subarray)
   - Update `max_sum` if `current_sum` is larger.

3. **Return** `max_sum`:
   - This represents the maximum subarray sum found.

---

## ✅ Constraints

- \( 1 \leq \text{nums.length} \leq 10^5 \)
- \( -10^4 \leq \text{nums}[i] \leq 10^4 \)

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |
