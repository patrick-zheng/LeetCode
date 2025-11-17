# 🧩 LeetCode Problem: Best Time to Buy and Sell Stock

- **Problem Link**: [Best Time to Buy and Sell Stock – LeetCode](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/solutions/)

---

## 🧠 Algorithm Explanation

We are given an array `prices` where `prices[i]` is the stock price on day `i`.  
We want to **buy once** and **sell once later** to maximize profit:

$$
\text{profit} = prices[\text{sell}] - prices[\text{buy}], \quad \text{sell} > \text{buy}
$$

Instead of checking all pairs `(buy, sell)` (which is O(n²)), we can do it in one pass using a greedy approach:

- As we scan from left to right, we keep track of the **minimum price seen so far**.  
- For each day `i`, we imagine **selling on day i** and compute:

$$
\text{profit\_today} = prices[i] - \text{min\_price\_so\_far}
$$

- We update the **maximum profit** with this value if it’s better.
- We also update `min_price_so_far` whenever we encounter a smaller price.

This guarantees:

- The **buy day is always before the sell day** (since `min_price_so_far` only comes from earlier days).
- We consider the best possible sell point for each possible earlier buy in a single pass.

If the price only goes down (no profit possible), the best answer is `0` (do nothing).

We use this greedy method because:

- It gives optimal result.
- It’s simple and runs in O(n) with O(1) extra space.
- The problem allows only **one** transaction, which fits perfectly with a single-pass min-tracking strategy.

---

### 🪜 Steps

1. **Step 1**:  
   Initialize:
   - `min_price` as `prices[0]` (the first day’s price).
   - `max_profit` as `0` (best profit so far).

2. **Step 2**:  
   Loop over the prices from day 1 to the end:
   - For the current price `price`, compute a candidate profit:  
     `profit = price - min_price`.
   - If `profit > max_profit`, update `max_profit = profit`.

3. **SStep 3**:  
   Still inside the loop:
   - If the current `price` is less than `min_price`, update `min_price = price`.
   - After finishing the loop, return `max_profit`.  
     - If all differences are negative, `max_profit` stays `0`, meaning no trade is better than losing money.

---

## ✅ Constraints

- `1 <= prices.length <= 10^5` (typical LeetCode constraint)
- `0 <= prices[i] <= 10^4`
- You may complete **at most one** transaction (buy once and sell once).
- Must buy **before** you sell (no selling in the past).

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
