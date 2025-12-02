class Solution:
    def minCut(self, s: str) -> int:
        n = len(s)
        if n <= 1:
            return 0

        # pal[i][j] = True if s[i..j] is a palindrome
        pal = [[False] * n for _ in range(n)]

        # Fill palindrome table
        for i in range(n - 1, -1, -1):
            for j in range(i, n):
                if s[i] == s[j] and (j - i < 2 or pal[i + 1][j - 1]):
                    pal[i][j] = True

        # dp[i] = min cuts needed for s[0..i]
        dp = [0] * n
        for i in range(n):
            # maximum cuts if we cut before every character
            min_cuts = i
            # if the whole prefix is palindrome, no cut needed
            if pal[0][i]:
                min_cuts = 0
            else:
                # try all possible previous cut points
                for j in range(i):
                    if pal[j + 1][i]:
                        min_cuts = min(min_cuts, dp[j] + 1)
            dp[i] = min_cuts

        return dp[-1]
