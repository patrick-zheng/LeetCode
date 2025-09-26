#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int maximalRectangle(vector<vector<char>>& matrix) {
        if (matrix.empty() || matrix[0].empty()) return 0;
        int rows = matrix.size(), cols = matrix[0].size();
        vector<int> heights(cols + 1, 0); // +1 sentinel zero
        int best = 0;

        for (int r = 0; r < rows; ++r) {
            // Update histogram heights for this row
            for (int c = 0; c < cols; ++c) {
                heights[c] = (matrix[r][c] == '1') ? heights[c] + 1 : 0;
            }

            // Largest Rectangle in Histogram with monotonic increasing stack of indices
            vector<int> st;
            for (int i = 0; i <= cols; ++i) {
                while (!st.empty() && heights[i] < heights[st.back()]) {
                    int h = heights[st.back()]; st.pop_back();
                    int left = st.empty() ? -1 : st.back();
                    int width = i - left - 1;
                    best = max(best, h * width);
                }
                st.push_back(i);
            }
        }
        return best;
    }
};