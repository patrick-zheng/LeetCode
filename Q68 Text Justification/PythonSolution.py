class Solution:
    def fullJustify(self, words: list[str], maxWidth: int) -> list[str]:
        res = []
        curr_line = []
        num_of_letters = 0

        for word in words:
            if num_of_letters + len(word) + len(curr_line) > maxWidth:
                for i in range(maxWidth - num_of_letters):
                    curr_line[i % (len(curr_line) - 1 or 1)] += ' '
                res.append(''.join(curr_line))
                curr_line, num_of_letters = [], 0
            curr_line.append(word)
            num_of_letters += len(word)

        return res + [' '.join(curr_line).ljust(maxWidth)]
    