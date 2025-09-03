#include <string>

using namespace std;

class Solution {
public:
    string addBinary(string a, string b) {
        string res;
        int i = (int)a.size() - 1, j = (int)b.size() - 1, carry = 0;

        while (i >= 0 || j >= 0 || carry) {
            int bitA = (i >= 0 ? a[i--] - '0' : 0);
            int bitB = (j >= 0 ? b[j--] - '0' : 0);
            int total = bitA + bitB + carry;
            res.push_back(char('0' + (total & 1)));
            carry = total >> 1;
        }

        reverse(res.begin(), res.end());
        return res;
    }
};
