class Solution:
    def maximalRectangle(self, matrix: list[list[str]]) -> int:
        if not matrix or not matrix[0]:
            return 0

        rows, cols = len(matrix), len(matrix[0])
        heights = [0] * (cols + 1)  # extra 0 as sentinel
        best = 0

        for r in range(rows):
            # build histogram for row r
            for c in range(cols):
                heights[c] = heights[c] + 1 if matrix[r][c] == '1' else 0

            # largest rectangle in histogram
            stack = []  # stores indices with increasing heights
            for i in range(cols + 1):
                while stack and heights[i] < heights[stack[-1]]:
                    h = heights[stack.pop()]
                    left = stack[-1] if stack else -1
                    width = i - left - 1
                    best = max(best, h * width)
                stack.append(i)

        return best
    