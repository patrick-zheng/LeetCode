#include <array>
#include <memory>
#include <string>
#include <vector>

using namespace std;

struct TrieNode {
    array<unique_ptr<TrieNode>, 26> children;
    string word;
};

class Solution {
public:
    vector<string> findWords(vector<vector<char>>& board, vector<string>& words) {
        auto root = make_unique<TrieNode>();
        for (const string& w : words) {
            TrieNode* node = root.get();
            for (char ch : w) {
                int c = ch - 'a';
                if (!node->children[c])
                    node->children[c] = make_unique<TrieNode>();
                node = node->children[c].get();
            }
            node->word = w;
        }

        m = static_cast<int>(board.size());
        n = static_cast<int>(board[0].size());
        ans.clear();
        for (int i = 0; i < m; ++i)
            for (int j = 0; j < n; ++j)
                dfs(board, i, j, root.get());
        return ans;
    }

private:
    vector<string> ans;
    int m = 0, n = 0;

    void dfs(vector<vector<char>>& board, int i, int j, TrieNode* parent) {
        if (i < 0 || i >= m || j < 0 || j >= n)
            return;
        char letter = board[i][j];
        if (letter == '#')
            return;
        int idx = letter - 'a';
        if (!parent->children[idx])
            return;
        TrieNode* curr = parent->children[idx].get();

        if (!curr->word.empty())
            ans.push_back(move(curr->word));

        board[i][j] = '#';
        dfs(board, i + 1, j, curr);
        dfs(board, i - 1, j, curr);
        dfs(board, i, j + 1, curr);
        dfs(board, i, j - 1, curr);
        board[i][j] = letter;

        bool leaf = true;
        for (const auto& child : curr->children)
            if (child) {
                leaf = false;
                break;
            }
        if (leaf && curr->word.empty())
            parent->children[idx].reset();
    }
};
