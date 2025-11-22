#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    bool isPalindrome(string s) {
        int i = 0, j = (int)s.size() - 1;

        auto isAlnum = [](char c) {
            return (c >= '0' && c <= '9') ||
                   (c >= 'a' && c <= 'z') ||
                   (c >= 'A' && c <= 'Z');
        };

        auto toLower = [](char c) {
            if (c >= 'A' && c <= 'Z') return char(c - 'A' + 'a');
            return c;
        };

        while (i < j) {
            while (i < j && !isAlnum(s[i])) ++i;
            while (i < j && !isAlnum(s[j])) --j;

            if (i >= j) break;

            if (toLower(s[i]) != toLower(s[j])) return false;

            ++i;
            --j;
        }
        return true;
    }
};
