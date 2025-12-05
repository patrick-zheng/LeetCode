# 🧩 LeetCode Problem: Candy

- **Problem Link**: [Candy – LeetCode](https://leetcode.com/problems/candy/)
- **Solution Link**: [Candy – LeetCode Solutions](https://leetcode.com/problems/candy/solutions/)

---

## 🧠 Algorithm Explanation

We must distribute candies so that:

- Every child gets **at least one** candy.
- Any child with a **higher rating** than an immediate neighbor gets **more candies** than that neighbor.

A single pass (left→right or right→left) can only enforce the rule with respect to one side.  
So we use a **two-pass greedy** approach:

- In a **left-to-right** pass, we ensure each child who has a higher rating than their **left** neighbor gets more candies than them.
- In a **right-to-left** pass, we ensure each child who has a higher rating than their **right** neighbor gets more candies than them.

We keep two arrays:

- `left[i]`: candies required to satisfy the left neighbor rule.
- `right[i]`: candies required to satisfy the right neighbor rule.

To satisfy both directions, for each child `i` we take:
`candies[i] = max(left[i], right[i])`.  
The sum of all `candies[i]` is the minimum total number of candies.

---

### 🪜 Steps

1. **Step 1**:  
   Initialize two arrays `left` and `right` of length `n` (number of children), and set all values to `1` (each child gets at least one candy).

2. **Step 2**:  
   Perform a **left-to-right** scan:
   - For each `i` from `1` to `n - 1`:
     - If `ratings[i] > ratings[i - 1]`, set `left[i] = left[i - 1] + 1`.
     - Otherwise, keep `left[i] = 1`.

3. **Step 3**:  
   Perform a **right-to-left** scan and combine:
   - For each `i` from `n - 2` down to `0`:
     - If `ratings[i] > ratings[i + 1]`, set `right[i] = right[i + 1] + 1`.
     - Otherwise, keep `right[i] = 1`.
   - For each index `i`, compute `candies[i] = max(left[i], right[i])`.
   - Return the sum of all `candies[i]`.

---

## ✅ Constraints

- `1 <= ratings.length <= 2 * 10^4`
- `ratings.length == n`
- `-10^9 <= ratings[i] <= 10^9`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |
