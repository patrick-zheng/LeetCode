#include <vector>
#include <array>
#include <string>
#include <functional>
#include <algorithm>

class Solution {
public:
    bool isScramble(const std::string& s1, const std::string& s2) {
        const int n = static_cast<int>(s1.size());
        if (n != static_cast<int>(s2.size())) return false;
        if (s1 == s2) return true;

        // Build 26-letter prefix counts for O(26) multiset checks
        auto buildPref = [&](const std::string& s) {
            std::vector<std::array<int, 26>> pref(n + 1);
            pref[0].fill(0);
            for (int i = 0; i < n; ++i) {
                pref[i + 1] = pref[i];
                pref[i + 1][s[i] - 'a']++;
            }
            return pref;
        };

        const auto pref1 = buildPref(s1);
        const auto pref2 = buildPref(s2);

        auto sameMultiset = [&](int i, int j, int len) {
            for (int k = 0; k < 26; ++k) {
                int c1 = pref1[i + len][k] - pref1[i][k];
                int c2 = pref2[j + len][k] - pref2[j][k];
                if (c1 != c2) return false;
            }
            return true;
        };

        // memo[i][j][len] = -1 unknown, 0 false, 1 true
        std::vector<std::vector<std::vector<int>>> memo(
            n, std::vector<std::vector<int>>(n, std::vector<int>(n + 1, -1)));

        std::function<bool(int, int, int)> dfs = [&](int i, int j, int len) -> bool {
            int& m = memo[i][j][len];
            if (m != -1) return m == 1;

            // Exact substring equality short-circuit
            if (std::equal(s1.begin() + i, s1.begin() + i + len, s2.begin() + j)) {
                m = 1;
                return true;
            }

            // Character multiset pruning
            if (!sameMultiset(i, j, len)) {
                m = 0;
                return false;
            }

            for (int k = 1; k < len; ++k) {
                // No swap
                if (dfs(i, j, k) && dfs(i + k, j + k, len - k)) {
                    m = 1; return true;
                }
                // Swap
                if (dfs(i, j + len - k, k) && dfs(i + k, j, len - k)) {
                    m = 1; return true;
                }
            }
            m = 0;
            return false;
        };

        return dfs(0, 0, n);
    }
};
