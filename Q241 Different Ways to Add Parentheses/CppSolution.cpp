#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
    unordered_map<string, vector<int>> memo_;

    vector<int> dfs(const string& s) {
        auto it = memo_.find(s);
        if (it != memo_.end()) {
            return it->second;
        }
        vector<int> res;
        bool has_op = false;
        for (int i = 0; i < static_cast<int>(s.size()); ++i) {
            char c = s[i];
            if (c != '+' && c != '-' && c != '*') {
                continue;
            }
            has_op = true;
            vector<int> left = dfs(s.substr(0, i));
            vector<int> right = dfs(s.substr(static_cast<size_t>(i) + 1));
            for (int a : left) {
                for (int b : right) {
                    if (c == '+') {
                        res.push_back(a + b);
                    } else if (c == '-') {
                        res.push_back(a - b);
                    } else {
                        res.push_back(a * b);
                    }
                }
            }
        }
        if (!has_op) {
            res.push_back(stoi(s));
        }
        memo_[s] = res;
        return res;
    }

public:
    vector<int> diffWaysToCompute(string expression) { return dfs(expression); }
};
