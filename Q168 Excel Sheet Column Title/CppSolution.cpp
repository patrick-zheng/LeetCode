#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    string convertToTitle(int columnNumber) {
        string res;

        while (columnNumber > 0) {
            columnNumber--;
            int rem = columnNumber % 26;
            res.push_back(char('A' + rem));
            columnNumber /= 26;
        }

        reverse(res.begin(), res.end());
        return res;
    }
};