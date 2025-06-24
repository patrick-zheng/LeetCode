class Solution:
    def myAtoi(self, s: str) -> int:
        MIN_INT: int = -2 ** 31
        MAX_INT: int = 2 ** 31 - 1 
        s = s.lstrip()
        if not s: return 0
        
        sign: int = 1
        result: int = 0
        counter: int = 0
        if s[0] == '-': 
            sign = -1
            counter += 1
        elif s[0] == '+': 
            sign = 1
            counter += 1
        
        while counter < len(s) and s[counter].isdigit():
            result = result * 10 + int(s[counter])
            counter += 1
            
        return max(MIN_INT, min(MAX_INT, sign * result))
