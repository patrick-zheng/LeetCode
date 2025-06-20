#include <unordered_map>
#include <string>

using namespace std;

class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        unordered_map<char, int> charMap;
        int left = 0;
        int maxLength = 0;

        for (int right = 0; right < s.length(); right++) {
            const char currentChar = s[right];
            if (charMap.find(currentChar) != charMap.end()) {
                left = max(left, charMap[currentChar] + 1);
            }
            charMap[currentChar] = right;
            maxLength = max(maxLength, right - left + 1);
        }

        return maxLength;
    }
};
