class Solution:
	def reverse(self, x: int) -> None:
		INT_MIN: int = -2 ** 31
		INT_MAX: int = 2 ** 31 - 1
		result: int = 0
		sign: int = -1 if x < 0 else 1
		n: int = abs(x)

		while n != 0:
			digit: int = n % 10
			n //= 10
			result = result * 10 + digit

		if (result > INT_MAX or result <= INT_MIN): return 0
		else: return sign * result
