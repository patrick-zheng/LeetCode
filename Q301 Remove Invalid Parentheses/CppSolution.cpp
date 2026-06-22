#include <functional>
#include <string>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
public:
    vector<string> removeInvalidParentheses(string s) {
        int leftRemove = 0;
        int rightRemove = 0;
        for (char ch : s) {
            if (ch == '(') {
                ++leftRemove;
            } else if (ch == ')') {
                if (leftRemove > 0) {
                    --leftRemove;
                } else {
                    ++rightRemove;
                }
            }
        }

        unordered_set<string> result;
        string path;

        function<void(int, int, int, int, int)> dfs =
            [&](int index, int openCount, int closeCount, int leftRem, int rightRem) {
                if (index == static_cast<int>(s.size())) {
                    if (leftRem == 0 && rightRem == 0) {
                        result.insert(path);
                    }
                    return;
                }

                char ch = s[index];
                if (ch != '(' && ch != ')') {
                    path.push_back(ch);
                    dfs(index + 1, openCount, closeCount, leftRem, rightRem);
                    path.pop_back();
                    return;
                }

                if (ch == '(' && leftRem > 0) {
                    dfs(index + 1, openCount, closeCount, leftRem - 1, rightRem);
                } else if (ch == ')' && rightRem > 0) {
                    dfs(index + 1, openCount, closeCount, leftRem, rightRem - 1);
                }

                if (ch == '(') {
                    path.push_back(ch);
                    dfs(index + 1, openCount + 1, closeCount, leftRem, rightRem);
                    path.pop_back();
                } else if (closeCount < openCount) {
                    path.push_back(ch);
                    dfs(index + 1, openCount, closeCount + 1, leftRem, rightRem);
                    path.pop_back();
                }
            };

        dfs(0, 0, 0, leftRemove, rightRemove);
        return vector<string>(result.begin(), result.end());
    }
};
