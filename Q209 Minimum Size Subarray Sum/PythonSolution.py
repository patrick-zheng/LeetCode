class Solution:
    def minSubArrayLen(self, target: int, nums: list[int]) -> int:
        left = 0
        curr_sum = 0
        ans = float("inf")

        for right, x in enumerate(nums):
            curr_sum += x
            while curr_sum >= target:
                ans = min(ans, right - left + 1)
                curr_sum -= nums[left]
                left += 1

        return 0 if ans == float("inf") else int(ans)
