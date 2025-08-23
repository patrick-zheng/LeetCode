# 🧩 LeetCode Problem: Permutation Sequence

- **Problem Link**: [Permutation Sequence – LeetCode](https://leetcode.com/problems/permutation-sequence/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/permutation-sequence/solutions/)

---

## 🧠 Algorithm Explanation

The goal is to find the k-th permutation of numbers `[1, 2, ..., n]` in lexicographical order without generating all permutations.  
We use the **factorial number system** (Cantor Expansion) to directly compute each position:

- There are `(n-1)!` permutations for each fixed first digit.
- `(k-1)` determines which "block" of permutations we are in.
- At each step, we:
  1. Select the correct digit by dividing `k-1` by `(n-1)!`.
  2. Remove that digit from the list of available numbers.
  3. Repeat for the next position using the remainder.

This avoids generating all permutations and gives an efficient O(n²) solution.

---

### 🪜 Steps

1. **Precompute factorials** up to `n` for block sizes.
2. **Create a list of numbers** `[1, 2, ..., n]`.
3. **Convert k to 0-based** (`k -= 1`) and iterate from `n` down to `1`:
   - `idx = k // (i-1)!` → pick the digit at index `idx`.
   - Remove that digit from the list.
   - `k %= (i-1)!` → move to the next block.
4. **Join selected digits** into the final permutation string.

---

## ✅ Constraints

- `1 <= n <= 9`
- `1 <= k <= n!`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n²)      |
| Space Complexity  | O(n)       |

---
