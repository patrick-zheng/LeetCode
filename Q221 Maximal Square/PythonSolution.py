class Solution:
    def maximalSquare(self, matrix: list[list[str]]) -> int:
        if not matrix or not matrix[0]:
            return 0

        rows, cols = len(matrix), len(matrix[0])
        # dp[j] = side length of largest all-'1' square ending at current row, column j
        dp = [0] * cols
        best_side = 0

        for i in range(rows):
            northwest = 0  # dp[i-1][j-1] for this column
            for j in range(cols):
                above = dp[j]
                if matrix[i][j] == "1":
                    left = dp[j - 1] if j else 0
                    dp[j] = min(above, left, northwest) + 1
                    best_side = max(best_side, dp[j])
                else:
                    dp[j] = 0
                northwest = above

        return best_side * best_side
