# 🧩 LeetCode Problem: Best Time to Buy and Sell Stock IV

- **Problem Link**: <https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/>
- **Solution Link**: <https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/solutions/>

---

## 🧠 Algorithm Explanation

You are given an integer `k` representing the maximum number of transactions and an array `prices`, where `prices[i]` is the price of a stock on day `i`.

Each transaction consists of:

- One **buy**
- One **sell**
- You must **sell before buying again**

The objective is to maximize total profit using **at most `k` transactions**.

---

### 🔎 Key Observations

1. If `k >= n/2` (where `n = prices.length`), the transaction constraint is effectively removed.
   - This becomes the **unlimited transactions** problem.
   - Simply sum every positive difference:

2. Otherwise, we must use **Dynamic Programming**.

Define:

Transition:

- Do nothing on day `i`:
- Sell on day `i`:

This avoids an inner loop and gives **O(nk)** time.

We compress the DP table into two 1D arrays to reduce space to **O(n)**.

---

## 🪜 Steps

1. If `prices` is empty or `k == 0`, return 0.
2. If `k >= n/2`, compute greedy unlimited-transactions profit.
3. Otherwise:
   - Initialize two arrays `prev` and `cur` of size `n` with 0.
   - For each transaction count `t` from `1` to `k`:
     - Set `best = -prices[0]`
     - For each day `i` from `1` to `n-1`:
     - Swap `prev` and `cur`
4. Return `prev[n-1]`.

---

## ✅ Constraints

- `1 <= k <= 100`
- `1 <= prices.length <= 1000`
- `0 <= prices[i] <= 1000`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   |  O(n · k)  |
| Space Complexity  |    O(n)    |

Where:

- `n` = number of days
- `k` = maximum number of transactions

If `k >= n/2`, time complexity reduces to **O(n)**.
