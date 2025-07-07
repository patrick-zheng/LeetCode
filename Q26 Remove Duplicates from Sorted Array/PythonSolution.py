class Solution:
    def removeDuplicates(self, nums: list[int]) -> int:
        write = 0
        for read in nums:
            if write == 0 or read != nums[write - 1]:
                nums[write] = read
                write += 1
        return write
