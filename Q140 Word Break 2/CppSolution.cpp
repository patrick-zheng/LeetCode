#include <string>
#include <vector>
#include <unordered_set>
#include <unordered_map>
#include <functional>
#include <algorithm>

using namespace std;

class Solution {
public:
    vector<string> wordBreak(string s, vector<string>& wordDict) {
        const int n = static_cast<int>(s.size());

        unordered_set<string> dict;
        dict.reserve(wordDict.size() * 2);
        int maxLen = 0;
        for (const auto& w : wordDict) {
            dict.insert(w);
            maxLen = max(maxLen, static_cast<int>(w.size()));
        }

        // DP prune: can[i] = whether s[i:] is breakable at all
        vector<char> can(n + 1, 0);
        can[n] = 1;
        for (int i = n - 1; i >= 0; --i) {
            const int end = min(n, i + maxLen);
            for (int j = i + 1; j <= end; ++j) {
                if (!can[j]) continue;
                if (dict.find(s.substr(i, j - i)) != dict.end()) {
                    can[i] = 1;
                    break;
                }
            }
        }

        unordered_map<int, vector<string>> memo;
        memo.reserve(n * 2);

        function<vector<string>(int)> dfs = [&](int i) -> vector<string> {
            if (!can[i]) return {};
            if (i == n) return {""};

            auto it = memo.find(i);
            if (it != memo.end()) return it->second;

            vector<string> res;
            const int end = min(n, i + maxLen);

            for (int j = i + 1; j <= end; ++j) {
                if (!can[j]) continue;

                string word = s.substr(i, j - i);
                if (dict.find(word) == dict.end()) continue;

                vector<string> tails = dfs(j);
                for (const auto& t : tails) {
                    if (t.empty()) res.push_back(word);
                    else res.push_back(word + " " + t);
                }
            }

            memo.emplace(i, res);
            return memo[i];
        };

        return dfs(0);
    }
};
