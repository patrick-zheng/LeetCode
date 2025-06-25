#include <string>

using namespace std;

class Solution {
public:
    int romanToInt(string s) {
        int roman[128] = {};
        roman['I'] = 1;
        roman['V'] = 5;
        roman['X'] = 10;
        roman['L'] = 50;
        roman['C'] = 100;
        roman['D'] = 500;
        roman['M'] = 1000;

        int total = 0;
        int prev = 0;

        for (int i = s.length() - 1; i >= 0; --i) {
            int curr = roman[s[i]];
            if (curr < prev) total -= curr;
            else total += curr;
            prev = curr;
        }

        return total;
    }
};
