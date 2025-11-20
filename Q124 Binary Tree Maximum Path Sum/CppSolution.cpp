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
    int maxPathSum(TreeNode* root) {
        int maxSum = INT_MIN;
        dfs(root, maxSum);
        return maxSum;
    }

private:
    int dfs(TreeNode* node, int& maxSum) {
        if (node == nullptr) return 0;

        // Best downward gains from left and right (ignore negatives)
        int leftGain  = std::max(0, dfs(node->left,  maxSum));
        int rightGain = std::max(0, dfs(node->right, maxSum));

        // Best path that passes *through* this node
        int pathThrough = node->val + leftGain + rightGain;

        // Update global maximum
        maxSum = std::max(maxSum, pathThrough);

        // Return best one-side gain to parent
        return node->val + std::max(leftGain, rightGain);
    }
};
