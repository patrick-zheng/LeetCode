#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    vector<string> generateParenthesis(int n) {
        vector<string> result;
        backtrack("", 0, 0, n, result);
        return result;
    }

private:
    void backtrack(string current, int openCount, int closeCount, int n, vector<string>& result) {
        if (current.length() == 2 * n) {
            result.push_back(current);
            return;
        }
        if (openCount < n) {
            backtrack(current + '(', openCount + 1, closeCount, n, result);
        }
        if (closeCount < openCount) {
            backtrack(current + ')', openCount, closeCount + 1, n, result);
        }
    }
};
