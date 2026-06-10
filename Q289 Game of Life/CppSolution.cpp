#include <vector>

using namespace std;

class Solution {
public:
    void gameOfLife(vector<vector<int>>& board) {
        int rows = static_cast<int>(board.size());
        int cols = static_cast<int>(board[0].size());

        for (int r = 0; r < rows; ++r) {
            for (int c = 0; c < cols; ++c) {
                int live_neighbors = 0;
                for (int nr = max(0, r - 1); nr < min(rows, r + 2); ++nr) {
                    for (int nc = max(0, c - 1); nc < min(cols, c + 2); ++nc) {
                        if (nr == r && nc == c) continue;
                        if (board[nr][nc] == 1 || board[nr][nc] == 2) {
                            ++live_neighbors;
                        }
                    }
                }

                if (board[r][c] == 1 && (live_neighbors < 2 || live_neighbors > 3)) {
                    board[r][c] = 2;
                } else if (board[r][c] == 0 && live_neighbors == 3) {
                    board[r][c] = 3;
                }
            }
        }

        for (int r = 0; r < rows; ++r) {
            for (int c = 0; c < cols; ++c) {
                if (board[r][c] == 2) {
                    board[r][c] = 0;
                } else if (board[r][c] == 3) {
                    board[r][c] = 1;
                }
            }
        }
    }
};
