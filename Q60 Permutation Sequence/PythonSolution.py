class Solution:
    def getPermutation(self, n: int, k: int) -> str:
        # Build factorials up to n
        fact = [1] * (n + 1)
        for i in range(1, n + 1):
            fact[i] = fact[i - 1] * i

        nums = list(range(1, n + 1))  # remaining digits
        k -= 1  # convert to 0-based index
        ans = []

        # Choose each position using factorial number system
        for i in range(n, 0, -1):
            block = fact[i - 1]
            idx = k // block
            ans.append(str(nums.pop(idx)))
            k %= block

        return "".join(ans)
