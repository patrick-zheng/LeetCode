#include <stack>

using namespace std;

class MyQueue {
    stack<int> in_stk;
    stack<int> out_stk;

    void move() {
        if (!out_stk.empty()) {
            return;
        }
        while (!in_stk.empty()) {
            out_stk.push(in_stk.top());
            in_stk.pop();
        }
    }

public:
    MyQueue() {}

    void push(int x) { in_stk.push(x); }

    int pop() {
        move();
        int x = out_stk.top();
        out_stk.pop();
        return x;
    }

    int peek() {
        move();
        return out_stk.top();
    }

    bool empty() { return in_stk.empty() && out_stk.empty(); }
};
