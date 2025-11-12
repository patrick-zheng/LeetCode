#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    vector<vector<int>> generate(int numRows) {
        vector<vector<int>> tri;
        tri.reserve(numRows);
        for (int r = 0; r < numRows; ++r) {
            if (r == 0) {
                tri.push_back({1});
            } else {
                const auto &prev = tri.back();
                vector<int> row;
                row.reserve((int)prev.size() + 1);
                row.push_back(1);
                for (int i = 0; i + 1 < (int)prev.size(); ++i) {
                    row.push_back(prev[i] + prev[i + 1]);
                }
                row.push_back(1);
                tri.push_back(std::move(row));
            }
        }
        return tri;
    }
};
