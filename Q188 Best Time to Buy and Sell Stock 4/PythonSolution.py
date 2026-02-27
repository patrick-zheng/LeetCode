class Solution:
    def maxProfit(self, k: int, prices: list[int]) -> int:
        n = len(prices)
        if n == 0 or k == 0:
            return 0

        # Unlimited transactions case
        if k >= n // 2:
            profit = 0
            for i in range(1, n):
                if prices[i] > prices[i - 1]:
                    profit += prices[i] - prices[i - 1]
            return profit

        prev = [0] * n
        cur = [0] * n

        for _ in range(1, k + 1):
            best = -prices[0]  # prev[0] - prices[0] == 0 - prices[0]
            for i in range(1, n):
                cur[i] = max(cur[i - 1], prices[i] + best)
                best = max(best, prev[i] - prices[i])
            prev, cur = cur, prev  # reuse arrays (cur will be overwritten next loop)

        return prev[-1]
