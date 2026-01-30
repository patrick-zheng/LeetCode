class Solution:
    def fractionToDecimal(self, numerator: int, denominator: int) -> str:
        if numerator == 0:
            return "0"

        res = []

        # Sign
        if (numerator < 0) ^ (denominator < 0):
            res.append("-")

        n, d = abs(numerator), abs(denominator)

        # Integer part
        res.append(str(n // d))
        rem = n % d
        if rem == 0:
            return "".join(res)

        res.append(".")

        # Remainder -> index in res-string where the digit for this remainder starts
        seen = {}

        while rem != 0:
            if rem in seen:
                idx = seen[rem]
                s = "".join(res)
                return s[:idx] + "(" + s[idx:] + ")"

            seen[rem] = len("".join(res))  # current output length (string index)

            rem *= 10
            res.append(str(rem // d))
            rem %= d

        return "".join(res)
    
