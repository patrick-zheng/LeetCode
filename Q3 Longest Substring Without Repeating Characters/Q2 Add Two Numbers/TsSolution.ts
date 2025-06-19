/*
 * Problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
 * Solution: https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */

function lengthOfLongestSubstring(s: string): number {
    const charMap = new Map<string, number>();
    let left = 0;
    let maxLength = 0;

    for (let right = 0; right < s.length; right++) {
        const currentChar = s[right];
        if (charMap.has(currentChar)) {
            left = Math.max(left, charMap.get(currentChar)! + 1);
        }
        charMap.set(currentChar, right);
        maxLength = Math.max(maxLength, right - left + 1);
    }

    return maxLength;
};
