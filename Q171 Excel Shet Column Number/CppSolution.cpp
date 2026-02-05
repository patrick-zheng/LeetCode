#include <string>

using namespace std;

class Solution {
public:
    int titleToNumber(string columnTitle) {
        long long result = 0;
        for (char c : columnTitle) {
            int val = (c - 'A') + 1;     // A->1 ... Z->26
            result = result * 26 + val;
        }
        return (int)result;
    }
};
