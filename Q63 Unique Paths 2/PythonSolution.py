class Solution:
    def uniquePathsWithObstacles(self, obstacleGrid: list[list[int]]) -> int:
        m, n = len(obstacleGrid), len(obstacleGrid[0])

        # If start or end is blocked, no paths.
        if obstacleGrid[0][0] == 1 or obstacleGrid[m-1][n-1] == 1:
            return 0

        dp = [0] * n
        dp[0] = 1  # start cell

        for i in range(m):
            for j in range(n):
                if obstacleGrid[i][j] == 1:
                    dp[j] = 0  # obstacle blocks paths to this cell
                elif j > 0:
                    dp[j] += dp[j - 1]  # from left + from top (already in dp[j])
        return dp[-1]
