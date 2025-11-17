class Solution:
    def maxProfit(self, prices: list[int]) -> int:
        if len(prices) < 2:
            return 0

        min_price = prices[0]
        max_profit = 0

        for price in prices[1:]:
            # If we sold today, what profit would we get?
            profit = price - min_price
            # Update maximum profit
            if profit > max_profit:
                max_profit = profit
            # Update minimum price seen so far
            if price < min_price:
                min_price = price

        return max_profit
    