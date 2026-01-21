class Solution:
    def findMin(self, nums: list[int]) -> int:
        l, r = 0, len(nums) - 1

        while l < r:
            mid = (l + r) // 2

            # Minimum is in the right half (mid is in the "left sorted" part)
            if nums[mid] > nums[r]:
                l = mid + 1
            else:
                # Minimum is at mid or in the left half
                r = mid

        return nums[l]
    