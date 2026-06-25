#include <algorithm>
#include <string>

using namespace std;

class Solution {
    static string addStrings(const string& a, const string& b) {
        string res;
        int i = static_cast<int>(a.size()) - 1;
        int j = static_cast<int>(b.size()) - 1;
        int carry = 0;

        while (i >= 0 || j >= 0 || carry) {
            int sum = carry;
            if (i >= 0) {
                sum += a[i--] - '0';
            }
            if (j >= 0) {
                sum += b[j--] - '0';
            }
            res.push_back(static_cast<char>('0' + sum % 10));
            carry = sum / 10;
        }

        reverse(res.begin(), res.end());
        return res;
    }

public:
    bool isAdditiveNumber(string num) {
        int n = static_cast<int>(num.size());

        auto validPair = [&](int i, int j) {
            if (num[0] == '0' && i > 1) {
                return false;
            }
            if (num[i] == '0' && j > i + 1) {
                return false;
            }

            string a = num.substr(0, i);
            string b = num.substr(i, j - i);
            int k = j;

            while (k < n) {
                string c = addStrings(a, b);
                if (k + static_cast<int>(c.size()) > n
                    || num.compare(k, c.size(), c) != 0) {
                    return false;
                }
                a = b;
                b = c;
                k += static_cast<int>(c.size());
            }
            return true;
        };

        for (int j = 1; j < n; ++j) {
            for (int i = 1; i < j; ++i) {
                if (validPair(i, j)) {
                    return true;
                }
            }
        }
        return false;
    }
};
