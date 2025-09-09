#include <string>
#include <vector>
#include <sstream>
using namespace std;

class Solution {
public:
    string simplifyPath(string path) {
        vector<string> st;
        string token;
        stringstream ss(path);
        while (getline(ss, token, '/')) {
            if (token.empty() || token == ".") continue;
            if (token == "..") {
                if (!st.empty()) st.pop_back();
            } else {
                st.push_back(token);
            }
        }
        if (st.empty()) return "/";
        string res;
        for (const auto& dir : st) {
            res.push_back('/');
            res += dir;
        }
        return res;
    }
};
