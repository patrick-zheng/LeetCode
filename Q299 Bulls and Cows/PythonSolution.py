class Solution:
    def getHint(self, secret: str, guess: str) -> str:
        bulls = 0
        secret_freq = [0] * 10
        guess_freq = [0] * 10

        for s, g in zip(secret, guess):
            if s == g:
                bulls += 1
            else:
                secret_freq[ord(s) - 48] += 1
                guess_freq[ord(g) - 48] += 1

        cows = sum(min(secret_freq[i], guess_freq[i]) for i in range(10))
        return f"{bulls}A{cows}B"
