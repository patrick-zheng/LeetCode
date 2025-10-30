#include <vector>
#include <string>

using namespace std;

// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

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
    TreeNode* sortedListToBST(ListNode* head) {
        int n = 0;
        for (ListNode* p = head; p; p = p->next) ++n;
        curr = head;
        return build(n);
    }

private:
    ListNode* curr = nullptr;

    TreeNode* build(int k) {
        if (k == 0) return nullptr;

        // left subtree of size k/2
        TreeNode* left = build(k / 2);

        // curr must be non-null here if n was counted correctly
        if (!curr) return nullptr; // defensive: should not happen
        TreeNode* root = new TreeNode(curr->val);
        root->left = left;

        curr = curr->next; // advance exactly once per node (inorder)

        // right subtree with remaining nodes
        root->right = build(k - 1 - (k / 2));
        return root;
    }
};
