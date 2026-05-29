class Solution:
    def numberToWords(self, num: int) -> str:
        if num == 0:
            return "Zero"

        below_20 = [
            "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
            "Nine", "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen",
            "Sixteen", "Seventeen", "Eighteen", "Nineteen",
        ]
        tens = [
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy",
            "Eighty", "Ninety",
        ]
        units = ["", "Thousand", "Million", "Billion"]

        def chunk_to_words(n: int) -> str:
            if n == 0:
                return ""
            if n < 20:
                return below_20[n]
            if n < 100:
                return tens[n // 10] + (
                    (" " + below_20[n % 10]) if n % 10 else ""
                )
            return below_20[n // 100] + " Hundred" + (
                (" " + chunk_to_words(n % 100)) if n % 100 else ""
            )

        parts: list[str] = []
        value = num
        for i in range(4):
            chunk = value % 1000
            if chunk:
                suffix = f" {units[i]}" if units[i] else ""
                parts.append(chunk_to_words(chunk) + suffix)
            value //= 1000

        return " ".join(reversed(parts))
