class Solution:
    def maxProduct(self, nums: list[int]) -> int:
        if not nums:
            return 0

        max_ending = nums[0]
        min_ending = nums[0]
        best = nums[0]

        for x in nums[1:]:
            if x < 0:
                max_ending, min_ending = min_ending, max_ending

            max_ending = max(x, x * max_ending)
            min_ending = min(x, x * min_ending)

            best = max(best, max_ending)

        return best
