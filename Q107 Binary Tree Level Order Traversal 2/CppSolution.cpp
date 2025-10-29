#include <vector>
#include <queue>

using namespace std;

// Definition for a binary tree node.
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
public:
    vector<vector<int>> levelOrderBottom(TreeNode* root) {
        if (!root) return {};
        queue<TreeNode*> q;
        q.push(root);
        vector<vector<int>> levels;
        while (!q.empty()) {
            int sz = q.size();
            vector<int> row;
            row.reserve(sz);
            while (sz--) {
                TreeNode* n = q.front(); q.pop();
                row.push_back(n->val);
                if (n->left)  q.push(n->left);
                if (n->right) q.push(n->right);
            }
            levels.push_back(move(row));
        }
        reverse(levels.begin(), levels.end());
        return levels;
    }
};
