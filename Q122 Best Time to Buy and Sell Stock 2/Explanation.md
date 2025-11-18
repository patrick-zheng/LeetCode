# 🧩 LeetCode Problem: Best Time to Buy and Sell Stock II

- **Problem Link**: [Best Time to Buy and Sell Stock II – LeetCode](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/)

---

## 🧠 Algorithm Explanation

Since you can trade as many times as you want (but hold **at most one share** at any time), the key observation is:

> **Any time the price goes up from one day to the next, you can profit from that increase.**

You don’t need to explicitly simulate every buy and sell. Instead:

- If `prices[i] > prices[i-1]`, you could:
  - buy at `prices[i-1]`
  - sell at `prices[i]`
- The profit from that small trade is `prices[i] - prices[i-1]`.
- Summing all such positive differences gives the **maximum** possible profit.

Same-day buy/sell permission doesn’t change the result here, because the price is constant within a day — you only have one price per day.

---

### 🪜 Steps

1. Initialize `profit = 0`.
2. Loop from day `1` to `n-1`:
   - If `prices[i] > prices[i-1]`, add `prices[i] - prices[i-1]` to `profit`.
3. Return `profit`.

---

## ✅ Constraints (Typical LeetCode Version)

- `1 <= prices.length <= 3 * 10^4`
- `0 <= prices[i] <= 10^4`

---

## ⏱ Time and Space Complexity

| Metric           | Complexity |
|------------------|------------|
| Time Complexity  | `O(n)`     |
| Space Complexity | `O(1)`     |

---
