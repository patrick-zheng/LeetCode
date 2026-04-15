class Solution:
    def containsNearbyAlmostDuplicate(self, nums: list[int], indexDiff: int, valueDiff: int) -> bool:
        if valueDiff < 0 or indexDiff < 0:
            return False
        width = valueDiff + 1
        buckets: dict[int, int] = {}
        for i, x in enumerate(nums):
            if i > indexDiff:
                left = nums[i - indexDiff - 1]
                del buckets[left // width]
            b = x // width
            if b in buckets:
                return True
            if b - 1 in buckets and x - buckets[b - 1] <= valueDiff:
                return True
            if b + 1 in buckets and buckets[b + 1] - x <= valueDiff:
                return True
            buckets[b] = x
        return False
