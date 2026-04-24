#include <string>

using namespace std;

class Solution {
public:
    int calculate(string s) {
        long long result = 0;
        long long lastNumber = 0;
        long long curr = 0;
        char op = '+';

        for (int i = 0; i < static_cast<int>(s.size()); ++i) {
            char c = s[i];
            if (c >= '0' && c <= '9') {
                curr = curr * 10 + (c - '0');
            }
            if (c == '+' || c == '-' || c == '*' || c == '/' || i == static_cast<int>(s.size()) - 1) {
                if (op == '+') {
                    result += lastNumber;
                    lastNumber = curr;
                } else if (op == '-') {
                    result += lastNumber;
                    lastNumber = -curr;
                } else if (op == '*') {
                    lastNumber *= curr;
                } else {
                    lastNumber = lastNumber / curr;
                }
                if (c == '+' || c == '-' || c == '*' || c == '/') {
                    op = c;
                }
                curr = 0;
            }
        }
        return static_cast<int>(result + lastNumber);
    }
};
