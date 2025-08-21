#include <string>
#include <sstream>
using namespace std;

class Solution {
public:
    int lengthOfLastWord(string s) {
        int end = s.size() - 1;
        while (end >= 0 && s[end] == ' ') end--;

        if (end < 0) return 0;

        int start = end;
        while (start >= 0 && s[start] != ' ') start--;

        return end - start;
    }
};
