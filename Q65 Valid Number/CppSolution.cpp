#include <iostream>
#include <string>
#include <algorithm>
#include <cctype>
#include <vector>

using namespace std;

class Solution {
public:
    bool isNumber(string s) {
        trim(s);
        if (s.empty()) return false;

        auto isDigits = [&](const string& t) {
            if (t.empty()) return false;
            for (char c : t) if (!isdigit(static_cast<unsigned char>(c))) return false;
            return true;
        };

        auto isInt = [&](string t, bool allowSign = true) {
            if (allowSign && !t.empty() && (t[0] == '+' || t[0] == '-'))
                t.erase(t.begin());
            return isDigits(t);
        };

        auto isDec = [&](string t, bool allowSign = true) {
            if (allowSign && !t.empty() && (t[0] == '+' || t[0] == '-'))
                t.erase(t.begin());
            if (count(t.begin(), t.end(), '.') != 1) return false;
            size_t pos = t.find('.');
            string left = t.substr(0, pos);
            string right = t.substr(pos + 1);
            if (left.empty() && right.empty()) return false;
            if (!left.empty() && !isDigits(left)) return false;
            if (!right.empty() && !isDigits(right)) return false;
            return true;
        };

        // check exponent
        int ePos = -1;
        for (int i = 0; i < (int)s.size(); ++i) {
            if (s[i] == 'e' || s[i] == 'E') {
                if (ePos != -1) return false;
                ePos = i;
            }
        }

        if (ePos != -1) {
            string base = s.substr(0, ePos);
            string exp  = s.substr(ePos + 1);
            if (base.empty() || exp.empty()) return false;
            bool baseOk = isInt(base) || isDec(base);
            bool expOk  = isInt(exp);
            return baseOk && expOk;
        }
        return isInt(s) || isDec(s);
    }

private:
    void trim(string &s) {
        auto notSpace = [](int ch){ return !isspace(ch); };
        s.erase(s.begin(), find_if(s.begin(), s.end(), notSpace));
        s.erase(find_if(s.rbegin(), s.rend(), notSpace).base(), s.end());
    }
};
