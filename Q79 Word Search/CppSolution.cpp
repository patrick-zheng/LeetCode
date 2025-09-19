#include <vector>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
public:
    bool exist(vector<vector<char>>& board, string word) {
        int R = board.size();
        if (R == 0) return false;
        int C = board[0].size();
        if (C == 0) return false;

        // --- Optimization 1: letter-count pruning ---
        unordered_map<char,int> bcnt, wcnt;
        for (auto& row : board) for (char ch : row) ++bcnt[ch];
        for (char ch : word) ++wcnt[ch];
        for (auto& [ch, cnt] : wcnt) if (bcnt[ch] < cnt) return false;

        // --- Optimization 2: search from the rarer end ---
        if (bcnt[word.front()] > bcnt[word.back()]) reverse(word.begin(), word.end());

        auto dfs = [&](auto&& self, int r, int c, int i) -> bool {
            if (board[r][c] != word[i]) return false;
            if (i == (int)word.size() - 1) return true;

            char tmp = board[r][c];
            board[r][c] = '#'; // mark visited

            static const int dr[4] = {1, -1, 0, 0};
            static const int dc[4] = {0, 0, 1, -1};
            for (int k = 0; k < 4; ++k) {
                int nr = r + dr[k], nc = c + dc[k];
                if (nr >= 0 && nr < R && nc >= 0 && nc < C && board[nr][nc] != '#') {
                    if (self(self, nr, nc, i + 1)) {
                        board[r][c] = tmp;
                        return true;
                    }
                }
            }

            board[r][c] = tmp; // restore
            return false;
        };

        char start = word[0];
        for (int r = 0; r < R; ++r) {
            for (int c = 0; c < C; ++c) {
                if (board[r][c] == start && dfs(dfs, r, c, 0)) return true;
            }
        }
        return false;
    }
};
