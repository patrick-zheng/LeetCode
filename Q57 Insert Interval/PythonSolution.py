class Solution:
    def insert(self, intervals: list[list[int]], newInterval: list[int]) -> list[list[int]]:
        s, e = newInterval
        i, n = 0, len(intervals)
        res = []

        # 1) Add everything that ends before newInterval starts
        while i < n and intervals[i][1] < s:
            res.append(intervals[i])
            i += 1

        # 2) Merge all intervals that overlap with [s, e]
        while i < n and intervals[i][0] <= e:
            s = min(s, intervals[i][0])
            e = max(e, intervals[i][1])
            i += 1
        res.append([s, e])

        # 3) Add the rest
        while i < n:
            res.append(intervals[i])
            i += 1

        return res
    