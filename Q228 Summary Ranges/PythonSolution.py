from typing import List


class Solution:
    def summaryRanges(self, nums: List[int]) -> List[str]:
        if not nums:
            return []

        ranges: List[str] = []
        start = nums[0]
        prev = nums[0]

        for i in range(1, len(nums)):
            if nums[i] == prev + 1:
                prev = nums[i]
                continue

            if start == prev:
                ranges.append(str(start))
            else:
                ranges.append(f"{start}->{prev}")

            start = nums[i]
            prev = nums[i]

        if start == prev:
            ranges.append(str(start))
        else:
            ranges.append(f"{start}->{prev}")

        return ranges
