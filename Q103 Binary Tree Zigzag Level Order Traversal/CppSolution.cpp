#include <vector>
#include <string>
#include <deque>

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
    vector<vector<int>> zigzagLevelOrder(TreeNode* root) {
        if (!root) return {};
        deque<TreeNode*> q;
        q.push_back(root);

        vector<vector<int>> res;
        bool left_to_right = true;

        while (!q.empty()) {
            int n = (int)q.size();
            vector<int> level(n);

            for (int i = 0; i < n; ++i) {
                TreeNode* node = q.front(); q.pop_front();
                int idx = left_to_right ? i : (n - 1 - i);
                level[idx] = node->val;

                if (node->left)  q.push_back(node->left);
                if (node->right) q.push_back(node->right);
            }
            res.push_back(std::move(level));
            left_to_right = !left_to_right;
        }
        return res;
    }
};
