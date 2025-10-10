#include <vector>
#include <unordered_map>

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
    vector<TreeNode*> generateTrees(int n) {
        if (n == 0) return {};
        memo.clear();
        return build(1, n);
    }

private:
    // Memo: (lo, hi) -> all unique BSTs for that interval
    unordered_map<long long, vector<TreeNode*>> memo;

    static long long key(int lo, int hi) {
        // pack two ints into one 64-bit key
        return (static_cast<long long>(lo) << 32) ^ static_cast<long long>(hi);
    }

    vector<TreeNode*> build(int lo, int hi) {
        if (lo > hi) return {nullptr};
        long long k = key(lo, hi);
        if (memo.count(k)) return memo[k];

        vector<TreeNode*> res;
        for (int root = lo; root <= hi; ++root) {
            auto lefts  = build(lo, root - 1);
            auto rights = build(root + 1, hi);
            for (auto* L : lefts) {
                for (auto* R : rights) {
                    TreeNode* node = new TreeNode(root);
                    node->left  = L;
                    node->right = R;
                    res.push_back(node);
                }
            }
        }
        memo[k] = res;
        return memo[k];
    }
};
