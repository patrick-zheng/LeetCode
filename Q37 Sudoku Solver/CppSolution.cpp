#include <vector>
#include <string>
#include <functional>

using namespace std;

class Solution {
public:
    void solveSudoku(vector<vector<char>>& board) {
        bool rows[9][9] = {}, cols[9][9] = {}, boxes[9][9] = {};
        vector<pair<int, int>> empty;

        // Preprocessing
        for (int r = 0; r < 9; ++r) {
            for (int c = 0; c < 9; ++c) {
                if (board[r][c] == '.') {
                    empty.emplace_back(r, c);
                } else {
                    int d = board[r][c] - '1';
                    rows[r][d] = cols[c][d] = boxes[(r / 3) * 3 + (c / 3)][d] = true;
                }
            }
        }

        function<bool(int)> solve = [&](int idx) -> bool {
            if (idx == empty.size()) return true;

            auto [r, c] = empty[idx];
            int b = (r / 3) * 3 + (c / 3);

            for (int d = 0; d < 9; ++d) {
                if (!rows[r][d] && !cols[c][d] && !boxes[b][d]) {
                    board[r][c] = '1' + d;
                    rows[r][d] = cols[c][d] = boxes[b][d] = true;

                    if (solve(idx + 1)) return true;

                    board[r][c] = '.';
                    rows[r][d] = cols[c][d] = boxes[b][d] = false;
                }
            }

            return false;
        };

        solve(0);
    }
};
