#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    int calculate(string s) {
        long res = 0;
        long num = 0;
        int sign = 1;
        vector<long> stack;

        for (char c : s) {
            if (c >= '0' && c <= '9') {
                num = num * 10 + (c - '0');
            } else if (c == '+') {
                res += sign * num;
                num = 0;
                sign = 1;
            } else if (c == '-') {
                res += sign * num;
                num = 0;
                sign = -1;
            } else if (c == '(') {
                stack.push_back(res);
                stack.push_back(sign);
                res = 0;
                sign = 1;
            } else if (c == ')') {
                res += sign * num;
                num = 0;
                long prevSign = stack.back();
                stack.pop_back();
                long prevRes = stack.back();
                stack.pop_back();
                res = prevRes + prevSign * res;
            }
        }

        return static_cast<int>(res + sign * num);
    }
};
