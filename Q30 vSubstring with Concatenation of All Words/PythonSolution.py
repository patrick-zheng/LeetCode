from collections import Counter

class Solution:
    def findSubstring(self, s: str, words: list[str]) -> list[int]:
        if not s or not words: return []
        
        wordLen = len(words[0])
        totalLen = wordLen * len(words)
        wordCount = Counter(words)
        result = []

        for i in range(wordLen):  # To handle offset positions
            left = i
            right = i
            windowCount = Counter()
            while right + wordLen <= len(s):
                word = s[right:right + wordLen]
                right += wordLen
                if word in wordCount:
                    windowCount[word] += 1
                    # Slide window if word count exceeds
                    while windowCount[word] > wordCount[word]:
                        windowCount[s[left:left + wordLen]] -= 1
                        left += wordLen
                    # All words matched
                    if right - left == totalLen:
                        result.append(left)
                else:
                    # Reset if invalid word
                    windowCount.clear()
                    left = right
        return result
    