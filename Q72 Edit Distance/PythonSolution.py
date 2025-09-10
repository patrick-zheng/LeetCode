class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        # Ensure word2 is the shorter to save space
        if len(word1) < len(word2):
            word1, word2 = word2, word1  # swap so len(word1) >= len(word2)

        m, n = len(word1), len(word2)
        dp = list(range(n + 1))  # dp[j] is distance for word1[:i] -> word2[:j] for current i

        for i in range(1, m + 1):
            prev_diag = dp[0]  # dp[i-1][0] before update
            dp[0] = i          # cost to convert first i chars to empty word2 (delete i chars)

            for j in range(1, n + 1):
                temp = dp[j]   # store old dp[j] == dp[i-1][j] (top cell)

                if word1[i - 1] == word2[j - 1]:
                    dp[j] = prev_diag
                else:
                    # replace: prev_diag + 1
                    # delete (from word1): temp + 1      (top cell + 1)
                    # insert (into word1): dp[j - 1] + 1 (left cell + 1)
                    dp[j] = 1 + min(prev_diag, temp, dp[j - 1])

                prev_diag = temp

        return dp[n]
    