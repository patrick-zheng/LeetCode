class Solution:
    def isInterleave(self, s1: str, s2: str, s3: str) -> bool:
        n, m = len(s1), len(s2)
        if n + m != len(s3):
            return False

        # Make s2 the shorter one to keep DP width small
        if m > n:
            s1, s2 = s2, s1
            n, m = m, n

        # dp[j] == can we form s3[:i+j] using s1[:i] and s2[:j]
        dp = [False] * (m + 1)
        dp[0] = True

        # Initialize first row: using only s2 to match s3
        for j in range(1, m + 1):
            dp[j] = dp[j - 1] and s2[j - 1] == s3[j - 1]

        # Fill DP row by row over s1
        for i in range(1, n + 1):
            # First column: using only s1 to match s3
            dp[0] = dp[0] and s1[i - 1] == s3[i - 1]
            for j in range(1, m + 1):
                take_s1 = dp[j] and s1[i - 1] == s3[i + j - 1]      # extend from above
                take_s2 = dp[j - 1] and s2[j - 1] == s3[i + j - 1]  # extend from left
                dp[j] = take_s1 or take_s2

        return dp[m]
