class Solution:
    def singleNumber(self, nums: list[int]) -> list[int]:
        xor_all = 0
        for x in nums:
            xor_all ^= x
        bit = xor_all & -xor_all
        a = b = 0
        for x in nums:
            if x & bit:
                a ^= x
            else:
                b ^= x
        return [a, b]
