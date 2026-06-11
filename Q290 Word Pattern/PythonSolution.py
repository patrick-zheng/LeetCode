class Solution:
    def wordPattern(self, pattern: str, s: str) -> bool:
        words = s.split()
        if len(pattern) != len(words):
            return False

        char_to_word: dict[str, str] = {}
        word_to_char: dict[str, str] = {}
        for ch, word in zip(pattern, words):
            if ch in char_to_word:
                if char_to_word[ch] != word:
                    return False
            elif word in word_to_char:
                return False
            else:
                char_to_word[ch] = word
                word_to_char[word] = ch
        return True
