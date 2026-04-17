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
    int countNodes(TreeNode* root) {
        if (root == nullptr) return 0;
        int leftDepth = leftDepthFrom(root->left);
        int rightDepth = leftDepthFrom(root->right);
        if (leftDepth == rightDepth) {
            return (1 << leftDepth) + countNodes(root->right);
        }
        return (1 << rightDepth) + countNodes(root->left);
    }

private:
    static int leftDepthFrom(TreeNode* node) {
        int d = 0;
        while (node != nullptr) {
            ++d;
            node = node->left;
        }
        return d;
    }
};
