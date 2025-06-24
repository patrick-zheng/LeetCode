#include <string>
#include <climits>

using namespace std;

class Solution {
public:
    int myAtoi(string s) {
        int i = 0, n = s.length();
        int result = 0, sign = 1;

        while (i < n && s[i] == ' ') i++;

        if (i < n && (s[i] == '+' || s[i] == '-')) {
            sign = (s[i] == '-') ? -1 : 1;
            i++;
        }

        while (i < n && isdigit(s[i])) {
            int digit = s[i++] - '0';

            if (result > INT_MAX / 10 || (result == INT_MAX / 10 && digit > INT_MAX % 10)) {
                return (sign == 1) ? INT_MAX : INT_MIN;
            }

            result = result * 10 + digit;
        }

        return result * sign;
    }
};
