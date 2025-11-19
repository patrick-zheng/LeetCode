from math import inf

class Solution:
    def maxProfit(self, prices: list[int]) -> int:
        if not prices:
            return 0

        # Initialize states
        buy1 = -inf   # Best profit after first buy
        sell1 = 0     # Best profit after first sell
        buy2 = -inf   # Best profit after second buy
        sell2 = 0     # Best profit after second sell

        for price in prices:
            # Order matters: update in reverse "time" of the transaction chain
            sell2 = max(sell2, buy2 + price)
            buy2 = max(buy2, sell1 - price)
            sell1 = max(sell1, buy1 + price)
            buy1 = max(buy1, -price)

        return int(sell2)
    