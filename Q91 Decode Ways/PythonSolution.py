class Solution:
    def numDecodings(self, s: str) -> int:
        if not s or s[0] == '0':
            return 0

        prev2, prev1 = 1, 1  # dp[-1]=1 (empty), dp[0]=1 if s[0] != '0'
        for i in range(1, len(s)):
            curr = 0
            if s[i] != '0':
                curr += prev1
            two = int(s[i-1:i+1])
            if 10 <= two <= 26:
                curr += prev2
            if curr == 0:
                return 0
            prev2, prev1 = prev1, curr
        return prev1
