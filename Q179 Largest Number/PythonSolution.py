from functools import cmp_to_key

class Solution:
    def largestNumber(self, nums: list[int]) -> str:
        strs = list(map(str, nums))

        def cmp(a: str, b: str) -> int:
            # Sort descending by (a+b) vs (b+a)
            if a + b > b + a:
                return -1
            if a + b < b + a:
                return 1
            return 0

        strs.sort(key=cmp_to_key(cmp))

        # If the largest is "0", all are zeros
        if strs[0] == "0":
            return "0"

        return "".join(strs)
