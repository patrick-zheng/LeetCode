import bisect


class Solution:
    def lengthOfLIS(self, nums: list[int]) -> int:
        tails: list[int] = []
        for num in nums:
            pos = bisect.bisect_left(tails, num)
            if pos == len(tails):
                tails.append(num)
            else:
                tails[pos] = num
        return len(tails)
