class Solution:
    def minPathSum(self, grid: list[list[int]]) -> int:
        if not grid or not grid[0]:
            return 0  # or raise ValueError("Empty grid")
        
        m, n = len(grid), len(grid[0])
        dp = [0] * n
        
        # Initialize first row
        dp[0] = grid[0][0]
        for j in range(1, n):
            dp[j] = dp[j - 1] + grid[0][j]
        
        # Process remaining rows
        for i in range(1, m):
            dp[0] += grid[i][0]  # first column: only from above
            for j in range(1, n):
                dp[j] = grid[i][j] + min(dp[j], dp[j - 1])
        
        return dp[-1]
