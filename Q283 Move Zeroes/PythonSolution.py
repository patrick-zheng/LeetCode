class Solution:
    def moveZeroes(self, nums: list[int]) -> None:
        write = 0
        for i in range(len(nums)):
            if nums[i] != 0:
                if i != write:
                    nums[write], nums[i] = nums[i], nums[write]
                write += 1
