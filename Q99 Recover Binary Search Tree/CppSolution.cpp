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
    void recoverTree(TreeNode* root) {
        TreeNode *first = nullptr, *second = nullptr, *prev = nullptr;
        TreeNode *curr = root;

        while (curr) {
            if (!curr->left) {
                // Visit curr
                detect(prev, curr, first, second);
                prev = curr;
                curr = curr->right;
            } else {
                // Find inorder predecessor
                TreeNode* pred = curr->left;
                while (pred->right && pred->right != curr) pred = pred->right;

                if (!pred->right) {
                    // Thread it
                    pred->right = curr;
                    curr = curr->left;
                } else {
                    // Remove thread & visit
                    pred->right = nullptr;
                    detect(prev, curr, first, second);
                    prev = curr;
                    curr = curr->right;
                }
            }
        }

        if (first && second) std::swap(first->val, second->val);
    }

private:
    static void detect(TreeNode* prev, TreeNode* curr,
                       TreeNode*& first, TreeNode*& second) {
        if (prev && prev->val > curr->val) {
            if (!first) first = prev;      // first inversion's left node
            second = curr;                  // last inversion's right node
        }
    }
};
