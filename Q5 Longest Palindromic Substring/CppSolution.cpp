#include <vector>
#include <string>
#include <algorithm>
#include <stdexcept>

using namespace std;

class Solution {
public:
    string longestPalindrome(string s) {
        if (s.empty()) return "";

        // Step 1: Preprocess the string with boundaries
        string t = "^#";
        for (char c : s) {
            t += c;
            t += '#';
        }
        t += '$';

        int n = t.size();
        vector<int> P(n, 0);
        int center = 0, right = 0;

        // Step 2: Manacher's algorithm
        for (int i = 1; i < n - 1; ++i) {
            int mirror = 2 * center - i;

            if (i < right)
                P[i] = min(right - i, P[mirror]);

            // Attempt to expand palindrome centered at i
            while (t[i + P[i] + 1] == t[i - P[i] - 1])
                P[i]++;

            // If expanded past right, update center and right
            if (i + P[i] > right) {
                center = i;
                right = i + P[i];
            }
        }

        // Step 3: Find the maximum element in P
        int maxLen = 0;
        int centerIndex = 0;
        for (int i = 1; i < n - 1; ++i) {
            if (P[i] > maxLen) {
                maxLen = P[i];
                centerIndex = i;
            }
        }

        // Step 4: Extract the longest palindromic substring from original string
        int start = (centerIndex - maxLen) / 2;
        return s.substr(start, maxLen);
    }
};
