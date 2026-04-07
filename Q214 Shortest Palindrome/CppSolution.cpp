#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    string shortestPalindrome(string s) {
        if (s.empty()) {
            return s;
        }
        const int n = static_cast<int>(s.size());
        string rev(s.rbegin(), s.rend());
        string combined = s + "#" + rev;
        const int m = static_cast<int>(combined.size());
        vector<int> lps(m, 0);
        for (int i = 1; i < m; ++i) {
            int j = lps[i - 1];
            while (j > 0 && combined[i] != combined[j]) {
                j = lps[j - 1];
            }
            if (combined[i] == combined[j]) {
                ++j;
            }
            lps[i] = j;
        }
        const int l = lps[m - 1];
        return rev.substr(0, n - l) + s;
    }
};
