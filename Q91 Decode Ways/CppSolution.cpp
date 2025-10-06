#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int numDecodings(string s) {
        if (s.empty() || s[0] == '0') return 0;

        long prev2 = 1, prev1 = 1; // ways up to i-2, i-1
        for (size_t i = 1; i < s.size(); ++i) {
            long curr = 0;
            if (s[i] != '0') curr += prev1;
            int two = (s[i-1]-'0')*10 + (s[i]-'0');
            if (10 <= two && two <= 26) curr += prev2;
            if (curr == 0) return 0;
            prev2 = prev1;
            prev1 = curr;
        }
        return (int)prev1;
    }
};
