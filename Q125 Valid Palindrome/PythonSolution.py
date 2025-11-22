class Solution:
    def isPalindrome(self, s: str) -> bool:
        # filtered_chars = [char.lower() for char in s if char.isalnum()]
        # return filtered_chars == filtered_chars[::-1]
        
        left, right = 0, len(s) - 1

        while left < right:
            while left < right and not s[left].isalnum(): left += 1
            while left < right and not s[right].isalnum(): right -= 1

            if left >= right: break
            if s[left].lower() != s[right].lower(): return False

            left += 1
            right -= 1

        return True
