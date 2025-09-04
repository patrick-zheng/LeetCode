#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    vector<string> fullJustify(vector<string>& words, int maxWidth) {
        vector<string> res;
        int n = (int)words.size();
        int i = 0;

        while (i < n) {
            int j = i, lineLen = 0;
            // Greedily take as many words as fit: letters + gaps (j - i)
            while (j < n && lineLen + (int)words[j].size() + (j - i) <= maxWidth) {
                lineLen += (int)words[j].size();
                ++j;
            }

            int gaps = j - i - 1;
            bool lastLine = (j == n);
            string line;

            if (gaps == 0 || lastLine) {
                // Left-justify
                line = words[i];
                for (int k = i + 1; k < j; ++k) {
                    line.push_back(' ');
                    line += words[k];
                }
                line += string(maxWidth - (int)line.size(), ' ');
            } else {
                // Fully justify
                int totalSpaces = maxWidth - lineLen;
                int base = totalSpaces / gaps;
                int extra = totalSpaces % gaps;

                for (int k = i; k < j - 1; ++k) {
                    line += words[k];
                    line += string(base + (extra > 0 ? 1 : 0), ' ');
                    if (extra > 0) --extra;
                }
                line += words[j - 1];
            }

            res.push_back(std::move(line));
            i = j;
        }

        return res;
    }
};
