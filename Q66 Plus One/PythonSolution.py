class Solution:
    def plusOne(self, digits: list[int]) -> list[int]:
        n = len(digits)
        carry = 1

        for i in range(n - 1, -1, -1):
            total = digits[i] + carry
            digits[i] = total % 10
            carry = total // 10

            if carry == 0:
                break

        if carry:
            digits.insert(0, carry)

        return digits
    