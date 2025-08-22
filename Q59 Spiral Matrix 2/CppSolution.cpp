#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    vector<vector<int>> generateMatrix(int n) {
        vector<vector<int>> matrix(n, vector<int>(n, 0));
        int left = 0, right = n - 1, top = 0, bottom = n - 1;
        int num = 1;

        while (left <= right && top <= bottom) {
            // top row: left -> right
            for (int c = left; c <= right; ++c) matrix[top][c] = num++;
            ++top;

            // right column: top -> bottom
            for (int r = top; r <= bottom; ++r) matrix[r][right] = num++;
            --right;

            // bottom row: right -> left
            if (top <= bottom) {
                for (int c = right; c >= left; --c) matrix[bottom][c] = num++;
                --bottom;
            }

            // left column: bottom -> top
            if (left <= right) {
                for (int r = bottom; r >= top; --r) matrix[r][left] = num++;
                ++left;
            }
        }

        return matrix;
    }
};

