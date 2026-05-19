#include <string>
#include <vector>

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
    void dfs(TreeNode* node, vector<int>& path, vector<string>& out) {
        if (!node) return;
        path.push_back(node->val);
        if (!node->left && !node->right) {
            string s = to_string(path[0]);
            for (size_t i = 1; i < path.size(); ++i) {
                s += "->" + to_string(path[i]);
            }
            out.push_back(s);
        } else {
            dfs(node->left, path, out);
            dfs(node->right, path, out);
        }
        path.pop_back();
    }

public:
    vector<string> binaryTreePaths(TreeNode* root) {
        vector<string> out;
        vector<int> path;
        dfs(root, path, out);
        return out;
    }
};
