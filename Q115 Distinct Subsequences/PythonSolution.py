class Solution:
    def numDistinct(self, s: str, t: str) -> int:
        m, n = len(s), len(t)
        if n == 0:
            return 1
        if m < n:
            return 0

        dp = [0] * (n + 1)
        dp[0] = 1  # empty t

        for ch in s:
            for j in range(n, 0, -1):
                if ch == t[j - 1]:
                    dp[j] += dp[j - 1]
        return dp[n]
