#include <vector>
#include <string>

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
    int sumNumbers(TreeNode* root) {
        return dfs(root, 0);
    }
    
private:
    int dfs(TreeNode* node, int current) {
        if (!node) return 0;

        // Build the number for this path
        current = current * 10 + node->val;

        // Leaf node → contributes a full number
        if (!node->left && !node->right) {
            return current;
        }

        // Sum from left and right
        return dfs(node->left, current) + dfs(node->right, current);
    }
};
