class Solution:
    def letterCombinations(self, digits: str) -> list[str]:
        digit_to_letters = {
            '2': ['a', 'b', 'c'],
            '3': ['d', 'e', 'f'],
            '4': ['g', 'h', 'i'],
            '5': ['j', 'k', 'l'],
            '6': ['m', 'n', 'o'],
            '7': ['p', 'q', 'r', 's'],
            '8': ['t', 'u', 'v'],
            '9': ['w', 'x', 'y', 'z']
        }

        if not digits:
            return []

        # Function to backtrack and build the combinations
        def backtrack(index: int, path: list[str]):
            # If the current path is the same length as the input digits, store the combination
            if index == len(digits):
                result.append(''.join(path))
                return

            # Get the current digit's corresponding letters
            current_digit = digits[index]
            for letter in digit_to_letters[current_digit]:
                # Add the current letter to the path and recurse
                path.append(letter)
                backtrack(index + 1, path)
                path.pop()  # Backtrack to explore other possibilities

        # List to hold all the resulting combinations
        result = []
        backtrack(0, [])
        return result
