class Solution:
    def calculate(self, s: str) -> int:
        result = 0
        last_number = 0
        curr = 0
        op = "+"

        for i, c in enumerate(s):
            if c.isdigit():
                curr = curr * 10 + (ord(c) - ord("0"))
            if c in "+-*/" or i == len(s) - 1:
                if op == "+":
                    result += last_number
                    last_number = curr
                elif op == "-":
                    result += last_number
                    last_number = -curr
                elif op == "*":
                    last_number *= curr
                else:  # '/'
                    last_number = int(last_number / curr)
                if c in "+-*/":
                    op = c
                curr = 0

        return result + last_number
