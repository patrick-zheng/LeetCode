import numpy as np


class Solution:
    def findKthLargest(self, nums: list[int], k: int) -> int:
        a = np.asarray(nums)
        i = len(a) - k
        return int(np.partition(a, i)[i])
