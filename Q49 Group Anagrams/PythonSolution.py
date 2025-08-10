from collections import defaultdict

class Solution:
    def groupAnagrams(self, strs: list[str]) -> list[list[str]]:
        anagram_map = defaultdict(list)
            
        for word in strs:
            count = [0] * 26  # Assuming all lowercase English letters
            for char in word:
                count[ord(char) - ord('a')] += 1
            anagram_map[tuple(count)].append(word)
        
        return list(anagram_map.values())
    