class Solution:
    def generate(self, numRows: int) -> list[list[int]]:
        triangle = []
        for r in range(numRows):
            if r == 0:
                triangle.append([1])
            else:
                prev = triangle[-1]
                # middle = pairwise sums of prev
                mid = [prev[i] + prev[i+1] for i in range(len(prev)-1)]
                triangle.append([1] + mid + [1])
        return triangle
