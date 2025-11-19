# 🧩 LeetCode Problem: Best Time to Buy and Sell Stock III

- **Problem Link**: [Best Time to Buy and Sell Stock III – LeetCode](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/solutions/)

---

## 🧠 Algorithm Explanation

You are given an array `prices` where `prices[i]` is the price of a stock on day `i`.  
You may complete **at most two** transactions (each transaction = 1 buy + 1 sell), and you **cannot** engage in multiple transactions at the same time (you must sell before you buy again).

We can model the process using four states that represent the best possible profit at each stage:

1. `buy1`  – best profit after the **first buy**
2. `sell1` – best profit after the **first sell**
3. `buy2`  – best profit after the **second buy**
4. `sell2` – best profit after the **second sell**

For each price `p` in `prices`, we update these states as follows:

- `buy1  = max(buy1, -p)`  
  Choose between keeping the previous `buy1` or buying for the first time today.

- `sell1 = max(sell1, buy1 + p)`  
  Choose between keeping the previous `sell1` or selling the first stock today.

- `buy2  = max(buy2, sell1 - p)`  
  Choose between keeping the previous `buy2` or buying the second stock using the profit from the first transaction.

- `sell2 = max(sell2, buy2 + p)`  
  Choose between keeping the previous `sell2` or selling the second stock today.

In the end, `sell2` holds the maximum profit achievable with at most two transactions.

This method is a **state-compressed DP**: it captures the full dynamic programming idea but only uses constant extra space.

---

### 🪜 Steps

1. **Initialize states**
   - `buy1  = -∞` (or a very large negative number)
   - `sell1 = 0`
   - `buy2  = -∞`
   - `sell2 = 0`

2. **Iterate through each price `p` in `prices`**
   - Update in this order (reverse of transaction order):
     1. `sell2 = max(sell2, buy2 + p)`
     2. `buy2  = max(buy2, sell1 - p)`
     3. `sell1 = max(sell1, buy1 + p)`
     4. `buy1  = max(buy1, -p)`

3. **Return the answer**
   - The result is `sell2`, the best profit with at most two transactions.

---

## ✅ Constraints

- `1 <= prices.length <= 10^5`
- `0 <= prices[i] <= 10^5`
- At most **two** complete transactions (buy + sell).
- You **must sell before** you can buy again (no overlapping transactions).

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
