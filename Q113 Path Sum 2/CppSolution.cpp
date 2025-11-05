#include <vector>
#include <string>
#include <functional>

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
    vector<vector<int>> pathSum(TreeNode* root, int targetSum) {
        vector<vector<int>> res;
        vector<int> path;

        function<void(TreeNode*, long long)> dfs = [&](TreeNode* node, long long rem) {
            if (!node) return;
            path.push_back(node->val);

            if (!node->left && !node->right && node->val == rem) {
                res.push_back(path);
            }

            dfs(node->left, rem - node->val);
            dfs(node->right, rem - node->val);

            path.pop_back();
        };

        dfs(root, targetSum);
        return res;
    }
};