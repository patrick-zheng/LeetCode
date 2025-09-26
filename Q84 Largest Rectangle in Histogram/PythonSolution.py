class Solution:
    def largestRectangleArea(self, heights: list[int]) -> int:
        h = [0] + heights + [0]
        stack = []
        max_area = 0

        for i, height in enumerate(h):
            while stack and h[stack[-1]] > height:
                top = stack.pop()
                width = i - stack[-1] - 1
                max_area = max(max_area, h[top] * width)
            stack.append(i)

        return max_area
