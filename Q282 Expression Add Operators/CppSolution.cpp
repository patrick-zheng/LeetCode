#include <string>
#include <vector>

using namespace std;

class Solution {
    vector<string> result;
    string num;
    string path;
    int target;
    int n;
    long long curr_val = 0;
    long long last_operand = 0;

    void try_push(long long value, int next_index, bool started) {
        string value_str = to_string(value);

        if (!started) {
            int mark = static_cast<int>(path.size());
            path += value_str;
            curr_val = value;
            last_operand = value;
            dfs(next_index, true);
            path.resize(mark);
            return;
        }

        struct Branch {
            char op;
            long long new_val;
            long long new_last;
        };
        Branch branches[] = {
            {'+', curr_val + value, value},
            {'-', curr_val - value, -value},
            {'*', curr_val - last_operand + last_operand * value, last_operand * value},
        };

        for (const auto& b : branches) {
            int mark = static_cast<int>(path.size());
            path.push_back(b.op);
            path += value_str;
            long long saved_val = curr_val;
            long long saved_last = last_operand;
            curr_val = b.new_val;
            last_operand = b.new_last;
            dfs(next_index, true);
            curr_val = saved_val;
            last_operand = saved_last;
            path.resize(mark);
        }
    }

    void dfs(int index, bool started) {
        if (index == n) {
            if (curr_val == target) {
                result.push_back(path);
            }
            return;
        }

        if (num[index] == '0') {
            try_push(0, index + 1, started);
            return;
        }

        long long value = 0;
        for (int end = index; end < n; ++end) {
            if (end > index && num[index] == '0') {
                break;
            }
            value = value * 10 + (num[end] - '0');
            try_push(value, end + 1, started);
        }
    }

public:
    vector<string> addOperators(string num, int target) {
        this->num = num;
        this->target = target;
        n = static_cast<int>(num.size());
        result.clear();
        path.clear();
        curr_val = 0;
        last_operand = 0;
        dfs(0, false);
        return result;
    }
};
