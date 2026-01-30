#include <string>
#include <unordered_map>
#include <cstdlib>

using namespace std;

class Solution {
public:
    string fractionToDecimal(int numerator, int denominator) {
        if (numerator == 0) return "0";

        long long n = (long long)numerator;
        long long d = (long long)denominator;

        string res;

        // sign
        if ((n < 0) ^ (d < 0)) res.push_back('-');

        n = llabs(n);
        d = llabs(d);

        // integer part
        res += to_string(n / d);
        long long rem = n % d;
        if (rem == 0) return res;

        res.push_back('.');

        // remainder -> index in res where digits for this remainder start
        unordered_map<long long, int> seen;

        while (rem != 0) {
            auto it = seen.find(rem);
            if (it != seen.end()) {
                res.insert(it->second, "(");
                res.push_back(')');
                break;
            }
            seen[rem] = (int)res.size();

            rem *= 10;
            res.push_back(char('0' + (rem / d)));
            rem %= d;
        }

        return res;
    }
};
