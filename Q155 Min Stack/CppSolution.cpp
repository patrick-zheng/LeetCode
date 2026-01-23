#include <vector>
#include <string>

using namespace std;

class MinStack {
private:
    vector<pair<int, int>> st;

public:
    MinStack() {}

    void push(int val) {
        if (st.empty()) {
            st.push_back({val, val});
        } else {
            int currentMin = st.back().second;
            st.push_back({val, min(val, currentMin)});
        }
    }

    void pop() {
        st.pop_back();
    }

    int top() {
        return st.back().first;
    }

    int getMin() {
        return st.back().second;
    }
};
