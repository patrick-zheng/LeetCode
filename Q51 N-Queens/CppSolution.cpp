#include <vector>
#include <string>
#include <functional>
using namespace std;

class Solution {
public:
    vector<vector<string>> solveNQueens(int n) {
        vector<vector<string>> result;
        vector<string> board(n, string(n, '.'));

        function<void(int, int, int, int)> backtrack = [&](int row, int cols, int diag1, int diag2) {
            if (row == n) {
                result.push_back(board);
                return;
            }

            int available = ((1 << n) - 1) & ~(cols | diag1 | diag2);
            while (available) {
                int bit = available & -available;
                available -= bit;
                int col = __builtin_ctz(bit);

                board[row][col] = 'Q';
                backtrack(row + 1, cols | bit, (diag1 | bit) << 1, (diag2 | bit) >> 1);
                board[row][col] = '.';
            }
        };

        backtrack(0, 0, 0, 0);
        return result;
    }
};
