class Solution:
    def shortestPalindrome(self, s: str) -> str:
        if not s:
            return s
        n = len(s)
        rev = s[::-1]
        combined = s + "#" + rev
        lps = [0] * len(combined)
        for i in range(1, len(combined)):
            j = lps[i - 1]
            while j > 0 and combined[i] != combined[j]:
                j = lps[j - 1]
            if combined[i] == combined[j]:
                j += 1
            lps[i] = j
        l = lps[-1]
        return rev[: n - l] + s
