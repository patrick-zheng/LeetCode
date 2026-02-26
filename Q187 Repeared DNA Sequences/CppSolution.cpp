#include <vector>
#include <string>
#include <unordered_set>

using namespace std;

class Solution {
public:
    vector<string> findRepeatedDnaSequences(string s) {
        int n = static_cast<int>(s.size());
        if (n < 10) return {};

        auto encode = [](char c) -> int {
            switch (c) {
                case 'A': return 0;
                case 'C': return 1;
                case 'G': return 2;
                case 'T': return 3;
                default:  return 0;
            }
        };

        const int MASK = (1 << 20) - 1;  // keep only last 20 bits
        int hash = 0;

        unordered_set<int> seen;
        unordered_set<int> added;
        vector<string> result;

        for (int i = 0; i < n; ++i) {
            hash = ((hash << 2) | encode(s[i])) & MASK;

            if (i >= 9) {
                if (seen.count(hash)) {
                    if (added.insert(hash).second) {
                        result.push_back(s.substr(i - 9, 10));
                    }
                } else {
                    seen.insert(hash);
                }
            }
        }

        return result;
    }
};
