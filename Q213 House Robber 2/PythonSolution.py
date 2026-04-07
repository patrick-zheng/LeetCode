class Solution:
    def rob(self, nums: list[int]) -> int:
        if len(nums) == 1:
            return nums[0]
        return max(self._rob_linear(nums[:-1]), self._rob_linear(nums[1:]))

    def _rob_linear(self, houses: list[int]) -> int:
        prev2, prev1 = 0, 0
        for money in houses:
            prev2, prev1 = prev1, max(prev1, prev2 + money)
        return prev1
