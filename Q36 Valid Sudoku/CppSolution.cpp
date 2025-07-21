#include <vector>

using namespace std;

class Solution {
public:
    bool isValidSudoku(vector<vector<char>>& board) {
        bool rows[9][9] = {};
        bool cols[9][9] = {};
        bool boxes[9][9] = {};

        for (int r = 0; r < 9; ++r) {
            for (int c = 0; c < 9; ++c) {
                char ch = board[r][c];
                if (ch == '.') continue;

                int num = ch - '1';  // map '1'-'9' to 0-8
                int boxIndex = (r / 3) * 3 + (c / 3);

                if (rows[r][num] || cols[c][num] || boxes[boxIndex][num])
                    return false;

                rows[r][num] = cols[c][num] = boxes[boxIndex][num] = true;
            }
        }

        return true;
    }
};
