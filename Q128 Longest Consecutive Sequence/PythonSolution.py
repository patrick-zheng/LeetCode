class Solution:
    def longestConsecutive(self, nums: list[int]) -> int:
        if not nums:
            return 0

        num_set = set(nums)
        longest = 0

        for x in num_set:
            if x - 1 not in num_set:
                current = x
                length = 1

                while current + 1 in num_set:
                    current += 1
                    length += 1

                longest = max(longest, length)

        return longest
