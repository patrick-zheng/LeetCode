class Solution:
    def searchMatrix(self, matrix: list[list[int]], target: int) -> bool:
        if not matrix or not matrix[0]:
            return False

        rows, cols = len(matrix), len(matrix[0])
        r, c = 0, cols - 1

        while r < rows and c >= 0:
            v = matrix[r][c]
            if v == target:
                return True
            if v > target:
                c -= 1
            else:
                r += 1

        return False
