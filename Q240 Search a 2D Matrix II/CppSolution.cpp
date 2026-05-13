#include <vector>

using namespace std;

class Solution {
public:
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        if (matrix.empty() || matrix[0].empty()) return false;
        int rows = static_cast<int>(matrix.size());
        int cols = static_cast<int>(matrix[0].size());
        int r = 0;
        int c = cols - 1;

        while (r < rows && c >= 0) {
            int v = matrix[r][c];
            if (v == target) return true;
            if (v > target) {
                --c;
            } else {
                ++r;
            }
        }
        return false;
    }
};
