#include <string>
#include <unordered_map>
#include <climits>

using namespace std;

class Solution {
public:
    string minWindow(string s, string t) {
        if (t.empty() || s.size() < t.size()) return "";

        unordered_map<char, int> need;
        for (char c : t) need[c]++;

        int remaining = static_cast<int>(t.size()); // total chars (with duplicates) still needed
        int bestLen = INT_MAX, bestL = 0;
        int l = 0;

        for (int r = 0; r < static_cast<int>(s.size()); ++r) {
            char cr = s[r];
            if (need.find(cr) != need.end()) {
                // use one occurrence of cr
                if (need[cr] > 0) remaining--;
                need[cr]--;
            }

            // Shrink from the left while the window is valid (remaining == 0)
            while (remaining == 0) {
                if (r - l + 1 < bestLen) {
                    bestLen = r - l + 1;
                    bestL = l;
                }
                char cl = s[l++];
                if (need.find(cl) != need.end()) {
                    need[cl]++;                 // put cl back
                    if (need[cl] > 0) remaining++; // window no longer valid
                }
            }
        }

        return bestLen == INT_MAX ? "" : s.substr(bestL, bestLen);
    }
};
