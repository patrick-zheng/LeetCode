class Solution:
    def isAdditiveNumber(self, num: str) -> bool:
        n = len(num)

        def valid_pair(i: int, j: int) -> bool:
            if num[0] == "0" and i > 1:
                return False
            if num[i] == "0" and j > i + 1:
                return False

            a, b = num[:i], num[i:j]
            k = j
            while k < n:
                c = str(int(a) + int(b))
                if not num.startswith(c, k):
                    return False
                a, b = b, c
                k += len(c)
            return True

        for j in range(1, n):
            for i in range(1, j):
                if valid_pair(i, j):
                    return True
        return False
