#include <vector>

using namespace std;

class NumArray {
    vector<int> nums;
    vector<int> tree;
    int n;

    void add(int i, int delta) {
        while (i <= n) {
            tree[i] += delta;
            i += i & -i;
        }
    }

    int prefix(int i) {
        int total = 0;
        while (i > 0) {
            total += tree[i];
            i -= i & -i;
        }
        return total;
    }

public:
    NumArray(vector<int>& nums) {
        this->nums = nums;
        n = static_cast<int>(nums.size());
        tree.assign(n + 1, 0);
        for (int i = 0; i < n; ++i) {
            add(i + 1, nums[i]);
        }
    }

    void update(int index, int val) {
        int delta = val - nums[index];
        nums[index] = val;
        add(index + 1, delta);
    }

    int sumRange(int left, int right) {
        return prefix(right + 1) - prefix(left);
    }
};
