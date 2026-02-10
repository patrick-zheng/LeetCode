class Solution:
    def calculateMinimumHP(self, dungeon: list[list[int]]) -> int:
        m, n = len(dungeon), len(dungeon[0])
        INF = 10**18

        # dp[j] = min health needed to ENTER current cell in current row at column j
        dp = [INF] * (n + 1)
        dp[n - 1] = 1  # sentinel to handle bottom-right cleanly

        for i in range(m - 1, -1, -1):
            dp[n] = INF  # right boundary sentinel for this row
            for j in range(n - 1, -1, -1):
                need_next = min(dp[j], dp[j + 1])   # down vs right
                need_here = need_next - dungeon[i][j]
                dp[j] = 1 if need_here <= 1 else need_here

        return dp[0]
