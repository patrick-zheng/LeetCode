#include <vector>
#include <string>
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
    vector<vector<int>> levelOrder(TreeNode* root) {
        vector<vector<int>> res;
        if (!root) return res;
        queue<TreeNode*> q;
        q.push(root);
        while (!q.empty()) {
            int k = q.size();
            vector<int> level; level.reserve(k);
            while (k--) {
                TreeNode* cur = q.front(); q.pop();
                level.push_back(cur->val);
                if (cur->left)  q.push(cur->left);
                if (cur->right) q.push(cur->right);
            }
            res.push_back(std::move(level));
        }
        return res;
    }
};
