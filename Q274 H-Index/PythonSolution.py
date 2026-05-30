class Solution:
    def hIndex(self, citations: list[int]) -> int:
        n = len(citations)
        bucket = [0] * (n + 1)
        for c in citations:
            if c >= n:
                bucket[n] += 1
            else:
                bucket[c] += 1

        papers = 0
        for h in range(n, -1, -1):
            papers += bucket[h]
            if papers >= h:
                return h
        return 0
