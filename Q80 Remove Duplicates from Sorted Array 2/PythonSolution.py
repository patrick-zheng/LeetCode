class Solution:
    def removeDuplicates(self, nums: list[int]) -> int:
        w = 0
        for x in nums:
            if w < 2 or x != nums[w - 2]:
                nums[w] = x
                w += 1
        return w
