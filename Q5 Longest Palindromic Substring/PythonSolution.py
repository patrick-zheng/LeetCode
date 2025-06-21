class Solution:
	def longestPalindrome(self, s: str) -> str:
		# Preprocess the string
		T = '^#' + '#'.join(s) + '#$'
		n = len(T)
		P = [0] * n
		c = r = 0

		for i in range(1, n - 1):
			mirror = 2 * c - i

			if i < r: P[i] = min(r - i, P[mirror])

			# Expand around center i
			while T[i + P[i] + 1] == T[i - P[i] - 1]: P[i] += 1

			# Update center and right boundary
			if i + P[i] > r:
				c = i
				r = i + P[i]

		# Find max length and its center
		max_len = max(P)
		center = P.index(max_len)
		start = (center - max_len) // 2  # Map back to original string
		return s[start:start + max_len]
