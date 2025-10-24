#include <vector>
#include <string>
#include <unordered_map>
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
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        if (preorder.empty()) return nullptr;
        unordered_map<int,int> in_pos;
        in_pos.reserve(inorder.size());
        for (int i = 0; i < (int)inorder.size(); ++i) in_pos[inorder[i]] = i;

        int pre = 0;
        function<TreeNode*(int,int)> build = [&](int lo, int hi) -> TreeNode* {
            if (lo > hi) return nullptr;
            int val = preorder[pre++];
            int mid = in_pos[val];
            TreeNode* root = new TreeNode(val);
            root->left  = build(lo, mid - 1);
            root->right = build(mid + 1, hi);
            return root;
        };

        return build(0, (int)inorder.size() - 1);
    }
};