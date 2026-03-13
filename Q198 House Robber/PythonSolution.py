class Solution:
    def rob(self, nums: list[int]) -> int:
        prev2 = 0  # max money up to house i-2
        prev1 = 0  # max money up to house i-1

        for money in nums:
            current = max(prev1, prev2 + money)
            prev2 = prev1
            prev1 = current

        return prev1
