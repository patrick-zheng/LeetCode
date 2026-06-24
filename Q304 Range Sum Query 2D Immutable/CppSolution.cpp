#include <vector>

using namespace std;

class NumMatrix {
    vector<vector<int>> prefix;

public:
    NumMatrix(vector<vector<int>>& matrix) {
        int rows = static_cast<int>(matrix.size());
        int cols = rows ? static_cast<int>(matrix[0].size()) : 0;
        prefix.assign(rows + 1, vector<int>(cols + 1, 0));

        for (int r = 0; r < rows; ++r) {
            for (int c = 0; c < cols; ++c) {
                prefix[r + 1][c + 1] = matrix[r][c] + prefix[r][c + 1]
                    + prefix[r + 1][c] - prefix[r][c];
            }
        }
    }

    int sumRegion(int row1, int col1, int row2, int col2) {
        return prefix[row2 + 1][col2 + 1] - prefix[row1][col2 + 1]
            - prefix[row2 + 1][col1] + prefix[row1][col1];
    }
};
