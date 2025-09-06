class Solution:
    def climbStairs(self, n: int) -> int:
        if n <= 2:
            return n

        one, two = 1, 2  # f(1), f(2)
        for i in range(3, n + 1):
            one, two = two, one + two

        return two
    