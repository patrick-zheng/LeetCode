class Solution:
    def maximumGap(self, nums: list[int]) -> int:
        if len(nums) < 2:
            return 0

        n = len(nums)
        min_val = min(nums)
        max_val = max(nums)

        if min_val == max_val:
            return 0

        # Bucket size (minimum possible max gap)
        import math
        bucket_size = math.ceil((max_val - min_val) / (n - 1))
        bucket_count = n - 1

        # Buckets
        bucket_min = [float('inf')] * bucket_count
        bucket_max = [-float('inf')] * bucket_count
        used = [False] * bucket_count

        # Fill buckets
        for num in nums:
            if num == min_val or num == max_val:
                continue
            idx = (num - min_val) // bucket_size
            bucket_min[idx] = min(bucket_min[idx], num)
            bucket_max[idx] = max(bucket_max[idx], num)
            used[idx] = True

        # Compute max gap
        prev = min_val
        max_gap = 0

        for i in range(bucket_count):
            if not used[i]:
                continue
            max_gap = max(max_gap, bucket_min[i] - prev)
            prev = bucket_max[i]

        # Final gap with max_val
        max_gap = max(max_gap, max_val - prev)

        return int(max_gap)
    