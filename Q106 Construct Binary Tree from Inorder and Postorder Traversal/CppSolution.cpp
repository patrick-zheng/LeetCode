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
    TreeNode* buildTree(vector<int>& inorder, vector<int>& postorder) {
        if (inorder.empty()) return nullptr;
        unordered_map<int,int> idx;
        idx.reserve(inorder.size());
        for (int i = 0; i < (int)inorder.size(); ++i) idx[inorder[i]] = i;

        int p = (int)postorder.size() - 1;  // postorder index (global cursor)

        function<TreeNode*(int,int)> build = [&](int lo, int hi) -> TreeNode* {
            if (lo > hi) return nullptr;
            int rootVal = postorder[p--];
            int mid = idx[rootVal];

            TreeNode* root = new TreeNode(rootVal);
            // Build right first, then left (since we consume postorder from the end)
            root->right = build(mid + 1, hi);
            root->left  = build(lo, mid - 1);
            return root;
        };

        return build(0, (int)inorder.size() - 1);
    }
};
