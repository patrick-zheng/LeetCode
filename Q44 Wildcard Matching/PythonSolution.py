class Solution:
    def isMatch(self, s: str, p: str) -> bool:
        m, n = len(p) + 1, len(s) + 1
        dp = [[False] * n for _ in range(m)]
        dp[0][0] = True

        for i in range(1, m):
            if p[i - 1] == '*':
                dp[i][0] = dp[i - 1][0]

        for i in range(1, m):
            pc = p[i - 1]
            for j in range(1, n):
                sc = s[j - 1]
                if pc == '?':
                    dp[i][j] = dp[i - 1][j - 1]
                elif pc == '*':
                    dp[i][j] = dp[i - 1][j] or dp[i][j - 1]
                else:
                    dp[i][j] = pc == sc and dp[i - 1][j - 1]

        return dp[-1][-1]
