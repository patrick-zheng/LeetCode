#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    vector<string> restoreIpAddresses(string s) {
        vector<string> res;
        string path;
        backtrack(0, 0, s, path, res);
        return res;
    }

private:
    void backtrack(int start, int parts, const string& s, string& path, vector<string>& res) {
        // If 4 parts chosen, the string must be fully consumed
        if (parts == 4) {
            if (start == (int)s.size()) res.push_back(path);
            return;
        }

        // Prune by remaining length constraints:
        // remaining characters must be between (4 - parts) and 3*(4 - parts)
        int remaining = s.size() - start;
        int need = 4 - parts;
        if (remaining < need || remaining > 3 * need) return;

        // Try segment lengths 1..3
        for (int len = 1; len <= 3 && start + len <= (int)s.size(); ++len) {
            // Leading zero rule: "0" is okay, but "01", "00", ... are not
            if (len > 1 && s[start] == '0') break;

            // Take substring and check <= 255
            int val = 0;
            for (int i = 0; i < len; ++i) {
                val = val * 10 + (s[start + i] - '0');
            }
            if (val > 255) break;

            // Append to path (with dot if not first part)
            int oldSize = path.size();
            if (parts > 0) path.push_back('.');
            path.append(s, start, len);

            backtrack(start + len, parts + 1, s, path, res);

            // Undo (backtrack)
            path.resize(oldSize);
        }
    }
};
