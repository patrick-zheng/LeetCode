#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    void solve(vector<vector<char>>& board) {
        int m = board.size();
        if (m == 0) return;
        int n = board[0].size();
        if (n == 0) return;

        auto bfs = [&](int r, int c) {
            queue<pair<int,int>> q;
            q.push({r, c});
            board[r][c] = 'E'; // escaped

            const int dirs[4][2] = {{-1,0}, {1,0}, {0,-1}, {0,1}};
            while (!q.empty()) {
                auto [x, y] = q.front();
                q.pop();
                for (auto &d : dirs) {
                    int nx = x + d[0];
                    int ny = y + d[1];
                    if (nx >= 0 && nx < m && ny >= 0 && ny < n && board[nx][ny] == 'O') {
                        board[nx][ny] = 'E';
                        q.push({nx, ny});
                    }
                }
            }
        };

        // 1. Mark border-connected 'O's as 'E'
        for (int i = 0; i < m; ++i) {
            if (board[i][0] == 'O') bfs(i, 0);
            if (board[i][n - 1] == 'O') bfs(i, n - 1);
        }
        for (int j = 0; j < n; ++j) {
            if (board[0][j] == 'O') bfs(0, j);
            if (board[m - 1][j] == 'O') bfs(m - 1, j);
        }

        // 2. Flip and restore
        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (board[i][j] == 'O') {
                    board[i][j] = 'X';
                } else if (board[i][j] == 'E') {
                    board[i][j] = 'O';
                }
            }
        }
    }
};
