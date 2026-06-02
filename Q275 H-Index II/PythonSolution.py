class Solution:
    def hIndex(self, citations: list[int]) -> int:
        n = len(citations)
        left, right = 0, n - 1
        answer = 0
        while left <= right:
            mid = (left + right) // 2
            h = n - mid
            if citations[mid] >= h:
                answer = h
                right = mid - 1
            else:
                left = mid + 1
        return answer
