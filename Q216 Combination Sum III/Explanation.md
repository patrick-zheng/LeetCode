# 🧩 LeetCode Problem: Combination Sum III

- **Problem Link**: [Combination Sum III – LeetCode](https://leetcode.com/problems/combination-sum-iii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/combination-sum-iii/solutions/)

---

## 🧠 Algorithm Explanation

We must list every **combination** of **exactly `k`** distinct digits from **`1` … `9`** whose sum is **`n`**. Order inside a combination does not matter, so we build combinations in **increasing order**: after choosing `start`, the next digit must be at least `start + 1`. That removes duplicate permutations of the same set.

This is **backtracking (DFS)** with **pruning**:

- **Feasible count**: from digit `d` we have `10 - d` digits left; if that is fewer than how many we still need, stop.
- **Positive sum**: if the next digit exceeds the remaining sum, larger digits will also fail, so break the loop.
- **Sum bounds** (optional but cheap): the smallest sum of `t` distinct digits starting at `d` is `d + (d+1) + … + (d+t-1)`. The largest is the sum of the `t` largest digits in `[d, 9]`. If `n` is outside `[min, max]` for the current state, skip recursing. With only nine digits, simple DFS is already tiny; these bounds reduce redundant calls further.

---

### 🪜 Steps

1. **DFS state**: next allowed digit `start`, how many picks remain `k_left`, remaining target sum `target`, and current path.
2. **Base case**: `k_left == 0` — if `target == 0`, record a copy of the path.
3. **Recursive step**: try each `i` from `start` to `9`; append `i`, recurse with `i + 1`, `k_left - 1`, `target - i`, then pop.
4. **Prune** using count, `i > target`, and min/max possible sums for the remaining picks.

---

## ✅ Constraints

- `2 <= k <= 9`
- `1 <= n <= 60`

---

## ⏱ Time and Space Complexity

In the worst case we explore subsets of `{1,…,9}` with pruning; `k` and `n` are small constants.

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(C(9, k)) in the combinatorial sense, with strong pruning in practice |
| Space Complexity  | O(k) for the recursion stack and path (output space excluded) |

---
